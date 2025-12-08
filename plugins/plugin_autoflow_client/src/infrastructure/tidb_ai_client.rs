use std::error::Error as StdError;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use futures::{Stream, StreamExt};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, error};

use super::utils::{
    apply_auth, build_http_client, map_reqwest_error, stream_timeout_seconds, trim_trailing_slash,
    truncate_debug,
};
use crate::domain::{
    AutoflowConfig, AutoflowError, AutoflowPort, ChatMessage, ChatRequest, ChatResponse,
    StreamChunk,
};

pub struct TiDBAIClient {
    state: Arc<RwLock<SharedState>>,
}

struct SharedState {
    http: Client,
    config: AutoflowConfig,
}

#[derive(Clone)]
struct ConfigSnapshot {
    base_url: String,
    api_key: Option<String>,
    default_engine: String,
    stream_timeout: Duration,
}

impl From<AutoflowConfig> for ConfigSnapshot {
    fn from(config: AutoflowConfig) -> Self {
        let base_url = trim_trailing_slash(&config.base_url);
        let stream_timeout = Duration::from_secs(stream_timeout_seconds(&config));

        Self {
            base_url,
            api_key: config.api_key,
            default_engine: config.default_engine,
            stream_timeout,
        }
    }
}

impl ConfigSnapshot {
    fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    fn chat_url(&self) -> String {
        format!("{}/chats", self.base_url)
    }

    fn delete_chat_url(&self, chat_id: &str) -> String {
        format!("{}/chats/{}", self.base_url, chat_id)
    }

    fn health_url(&self) -> String {
        format!("{}/healthz", self.base_url)
    }

    fn default_engine(&self) -> &str {
        &self.default_engine
    }

    fn stream_timeout(&self) -> Duration {
        self.stream_timeout
    }
}

impl TiDBAIClient {
    pub fn new(config: AutoflowConfig) -> Self {
        let client = build_http_client(&config).expect("failed to build http client");
        let state = SharedState {
            http: client,
            config,
        };

        Self {
            state: Arc::new(RwLock::new(state)),
        }
    }

    async fn context(&self) -> HttpContext {
        let state = self.state.read().await;
        HttpContext::new(state.http.clone(), state.config.clone())
    }
}

#[async_trait]
impl AutoflowPort for TiDBAIClient {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AutoflowError> {
        let ctx = self.context().await;
        let request = ctx.prepare_chat_request(request, false);
        let requested_chat_id = request.chat_id.clone();
        let response = ctx.send(ctx.chat_builder(false).json(&request)).await?;
        let raw_body = response.text().await.map_err(map_reqwest_error)?;

        debug!(target: "autoflow::chat", raw_len = raw_body.len(), "received chat response body");
        debug!(target: "autoflow::chat", "raw body: {}", raw_body);
        let payload = parse_chat_payload(&raw_body)?;
        debug!(target: "autoflow::chat", ?payload, "parsed chat payload");
        let response = to_chat_response(payload, requested_chat_id)?;

        debug!(target: "autoflow::chat", ?response, "constructed chat response");
        Ok(response)
    }

    async fn chat_stream(&self, request: ChatRequest) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk, AutoflowError>> + Send>>, AutoflowError> {
        let ctx = self.context().await;
        let request = ctx.prepare_chat_request(request, true);
        
        debug!(target: "autoflow::stream", "sending streaming chat request");
        let response = ctx.send(ctx.chat_builder(true).json(&request)).await?;

        let status = response.status();
        debug!(target: "autoflow::stream", ?status, "stream response accepted");
        for (key, value) in response.headers().iter() {
            match value.to_str() {
                Ok(val) => {
                    debug!(target: "autoflow::stream", header = %key, value = %val, "response header");
                }
                Err(_) => {
                    debug!(target: "autoflow::stream", header = %key, "response header (non utf-8 value)");
                }
            }
        }

        let stream = response.bytes_stream();
        let (tx, rx) = mpsc::unbounded_channel();

        spawn_stream_processor(
            stream,
            tx,
            request.chat_id.clone(),
            request.chat_name.clone(),
        );

        Ok(Box::pin(UnboundedReceiverStream::new(rx)))
    }

    async fn delete_chat(&self, chat_id: &str) -> Result<(), AutoflowError> {
        let ctx = self.context().await;
        ctx.send(ctx.delete_chat_builder(chat_id)).await?;
        Ok(())
    }

    async fn health_check(&self) -> Result<bool, AutoflowError> {
        let ctx = self.context().await;
        let response = ctx.send(ctx.health_builder()).await?;
        Ok(response.status().is_success())
    }

    async fn update_config(&self, new_config: AutoflowConfig) -> Result<(), AutoflowError> {
        let new_client = build_http_client(&new_config).map_err(AutoflowError::HttpError)?;

        let mut state = self.state.write().await;
        state.http = new_client;
        state.config = new_config;

        Ok(())
    }

    async fn get_config(&self) -> AutoflowConfig {
        self.state.read().await.config.clone()
    }
}

#[derive(Clone)]
struct HttpContext {
    client: Client,
    config: ConfigSnapshot,
}

impl HttpContext {
    fn new(client: Client, config: AutoflowConfig) -> Self {
        Self {
            client,
            config: ConfigSnapshot::from(config),
        }
    }

    fn prepare_chat_request(&self, mut request: ChatRequest, stream: bool) -> ChatRequest {
        request
            .chat_engine
            .get_or_insert_with(|| self.config.default_engine().to_string());
        request.stream = stream;
        request
    }

    fn chat_builder(&self, stream: bool) -> RequestBuilder {
        let builder = self.client.post(self.config.chat_url());
        let builder = if stream {
            builder
                .header("Accept", "text/event-stream")
                .timeout(self.config.stream_timeout())
        } else {
            builder.header("Accept", "application/json")
        };
        apply_auth(builder, self.config.api_key())
    }

    fn delete_chat_builder(&self, chat_id: &str) -> RequestBuilder {
        let builder = self
            .client
            .delete(self.config.delete_chat_url(chat_id))
            .header("Accept", "application/json");
        apply_auth(builder, self.config.api_key())
    }

    fn health_builder(&self) -> RequestBuilder {
        let builder = self
            .client
            .get(self.config.health_url())
            .header("Accept", "application/json");
        apply_auth(builder, self.config.api_key())
    }

    async fn send(&self, builder: RequestBuilder) -> Result<Response, AutoflowError> {
        let response = builder.send().await.map_err(map_reqwest_error)?;
        let status = response.status();

        if status == StatusCode::UNAUTHORIZED {
            return Err(AutoflowError::AuthenticationFailed);
        }

        if !status.is_success() {
            let body = response.text().await.map_err(map_reqwest_error)?;
            return Err(AutoflowError::InvalidResponse(body));
        }

        Ok(response)
    }
}

fn spawn_stream_processor<S>(
    mut stream: S,
    tx: mpsc::UnboundedSender<Result<StreamChunk, AutoflowError>>,
    chat_id: Option<String>,
    chat_name: Option<String>,
) where
    S: Stream<Item = Result<Bytes, reqwest::Error>> + Send + Unpin + 'static,
{
    tokio::spawn(async move {
        let mut processor = StreamProcessor::new(tx, chat_id, chat_name);
        while let Some(item) = stream.next().await {
            if let Err(err) = processor.process_item(item) {
                processor.emit_error(err);
                return;
            }
        }
        processor.finish();
    });
}

struct StreamProcessor {
    buffer: BytesMut,
    finished_sent: bool,
    conversation: ConversationState,
    tx: mpsc::UnboundedSender<Result<StreamChunk, AutoflowError>>,
}

impl StreamProcessor {
    fn new(
        tx: mpsc::UnboundedSender<Result<StreamChunk, AutoflowError>>,
        chat_id: Option<String>,
        chat_name: Option<String>,
    ) -> Self {
        Self {
            buffer: BytesMut::new(),
            finished_sent: false,
            conversation: ConversationState::new(chat_id, chat_name),
            tx,
        }
    }

    fn process_item(&mut self, item: Result<Bytes, reqwest::Error>) -> Result<(), AutoflowError> {
        match item {
            Ok(bytes) => self.process_bytes(bytes),
            Err(err) => {
                error!(target: "autoflow::stream", error = %err, "stream read error");
                if let Some(source) = StdError::source(&err) {
                    debug!(target: "autoflow::stream", source = %source, "stream read error source");
                }
                Err(AutoflowError::StreamError(err.to_string()))
            }
        }
    }

    fn process_bytes(&mut self, bytes: Bytes) -> Result<(), AutoflowError> {
        debug!(target: "autoflow::stream", bytes = bytes.len(), "received stream bytes");
        self.buffer.extend_from_slice(&bytes);

        while let Some(pos) = self.buffer.iter().position(|&b| b == b'\n') {
            let mut line = self.buffer.split_to(pos + 1);
            while line.ends_with(&[b'\n']) || line.ends_with(&[b'\r']) {
                line.truncate(line.len().saturating_sub(1));
            }

            if line.is_empty() {
                continue;
            }

            let line_str = std::str::from_utf8(&line).map_err(|err| {
                error!(target: "autoflow::stream", error = %err, "invalid utf-8 in stream line");
                AutoflowError::InvalidResponse(format!("invalid utf-8 data in stream: {err}"))
            })?;

            let trimmed = line_str.trim();
            if trimmed.is_empty() {
                continue;
            }

            debug!(target: "autoflow::stream", line = %truncate_debug(trimmed), "processing stream line");

            // 兼容 Autoflow tag:payload 和标准 SSE event/data 格式
            if let Some((_tag, _payload)) = trimmed.split_once(':') {
                // Autoflow 协议
                self.handle_line(trimmed)?;
            } else if trimmed.starts_with("event:") || trimmed.starts_with("data:") {
                // 简单 SSE 兼容：收集 event/data 字段
                // 这里只处理单行 event/data，若有多行可扩展为状态机
                let mut _event_type: Option<&str> = None;
                let mut data: Option<&str> = None;
                for part in trimmed.split('\n') {
                    if let Some(ev) = part.strip_prefix("event:") {
                        _event_type = Some(ev.trim());
                    } else if let Some(da) = part.strip_prefix("data:") {
                        data = Some(da.trim());
                    }
                }
                if let Some(data) = data {
                    // 你可以根据 event_type 做更细致的分发
                    self.handle_line(data)?;
                }
            } else {
                // 兜底：直接按 Autoflow 处理
                self.handle_line(trimmed)?;
            }
        }

        Ok(())
    }

    fn handle_line(&mut self, line: &str) -> Result<(), AutoflowError> {
        match line.split_once(':') {
            Some(("0", payload)) => self.handle_text_chunk(payload),
            Some(("2", payload)) => self.handle_chat_envelopes(payload),
            Some(("8", payload)) => self.handle_state_envelopes(payload),
            Some((unknown, payload)) => {
                debug!(
                    target: "autoflow::stream",
                    tag = unknown,
                    payload = %truncate_debug(payload),
                    "unknown stream tag",
                );
                Ok(())
            }
            None => {
                debug!(target: "autoflow::stream", line = %truncate_debug(line), "line without delimiter");
                Ok(())
            }
        }
    }

    fn handle_text_chunk(&mut self, payload: &str) -> Result<(), AutoflowError> {
        let text: String = serde_json::from_str(payload).map_err(|err| {
            error!(target: "autoflow::stream", error = %err, payload = %truncate_debug(payload), "failed to parse text chunk");
            AutoflowError::InvalidResponse(format!("failed to parse text chunk: {err}"))
        })?;

        debug!(
            target: "autoflow::stream",
            text_len = text.len(),
            finished_sent = self.finished_sent,
            "parsed text chunk",
        );

        self.emit_text_chunk(text);
        Ok(())
    }

    fn handle_chat_envelopes(&mut self, payload: &str) -> Result<(), AutoflowError> {
        let envelopes: Vec<TiDbChatEnvelope> = serde_json::from_str(payload).map_err(|err| {
            error!(target: "autoflow::stream", error = %err, payload = %truncate_debug(payload), "failed to parse chat envelopes");
            AutoflowError::InvalidResponse(format!("failed to parse chat envelope: {err}"))
        })?;

        debug!(
            target: "autoflow::stream",
            envelope_count = envelopes.len(),
            "processed chat envelopes",
        );

        for envelope in envelopes {
            if let Some(chat) = envelope.chat {
                self.update_conversation(chat.id, chat.title);
            }

            if let Some(assistant) = envelope.assistant_message {
                if assistant.finished_at.is_some() {
                    self.emit_finished();
                }
            }
        }

        Ok(())
    }

    fn handle_state_envelopes(&mut self, payload: &str) -> Result<(), AutoflowError> {
        let states: Vec<TiDbStateEnvelope> = serde_json::from_str(payload).map_err(|err| {
            error!(target: "autoflow::stream", error = %err, payload = %truncate_debug(payload), "failed to parse state envelopes");
            AutoflowError::InvalidResponse(format!("failed to parse state envelope: {err}"))
        })?;

        debug!(
            target: "autoflow::stream",
            envelope_count = states.len(),
            "processed state envelopes",
        );

        // 合并遍历：遇到 FINISHED emit_finished，否则 emit 状态 StreamChunk
        for state in &states {
            if let Some(state_name) = &state.state {
                if state_name == "FINISHED" {
                    self.emit_finished();
                } else {
                    let chunk = self.conversation.as_chunk(state_name.clone(), false);
                    self.send_chunk(chunk);
                }
            }
        }

        Ok(())
    }

    fn update_conversation(
        &mut self,
        conversation_id: Option<String>,
        conversation_name: Option<String>,
    ) {
        if let Some(id) = conversation_id {
            debug!(target: "autoflow::stream", conversation_id = %id, "updated conversation id");
            self.conversation.set_id(id);
        }

        if let Some(name) = conversation_name {
            debug!(target: "autoflow::stream", conversation_name = %name, "updated conversation name");
            self.conversation.set_name(name);
        }
    }

    fn emit_text_chunk(&self, text: String) {
        if text.is_empty() {
            return;
        }

        let chunk = self.conversation.as_chunk(text, false);
        self.send_chunk(chunk);
    }

    fn emit_finished(&mut self) {
        if self.finished_sent {
            return;
        }

        debug!(target: "autoflow::stream", "emitting finished chunk");
        self.finished_sent = true;
        let chunk = self.conversation.as_chunk(String::new(), true);
        self.send_chunk(chunk);
    }

    fn send_chunk(&self, chunk: StreamChunk) {
        debug!(
            target: "autoflow::stream",
            text_len = chunk.text.len(),
            finished = chunk.finished,
            "emitting stream chunk",
        );
        let _ = self.tx.send(Ok(chunk));
    }

    fn emit_error(&self, err: AutoflowError) {
        error!(target: "autoflow::stream", error = %err, "stream processor error");
        let _ = self.tx.send(Err(err));
    }

    fn flush_tail(&mut self) -> Result<(), AutoflowError> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let tail = std::str::from_utf8(&self.buffer).map_err(|err| {
            error!(target: "autoflow::stream", error = %err, "invalid utf-8 in tail line");
            AutoflowError::InvalidResponse(format!("invalid utf-8 data in stream: {err}"))
        })?;

        let trimmed = tail.trim();
        if trimmed.is_empty() {
            self.buffer.clear();
            return Ok(());
        }

        let owned = trimmed.to_owned();
        self.buffer.clear();
        debug!(
            target: "autoflow::stream",
            line = %truncate_debug(&owned),
            "processing tail line",
        );
        self.handle_line(&owned)
    }

    fn finish(&mut self) {
        if let Err(err) = self.flush_tail() {
            self.emit_error(err);
            return;
        }

        self.emit_finished();
    }
}

#[derive(Clone, Default)]
struct ConversationState {
    id: Option<String>,
    name: Option<String>,
}

impl ConversationState {
    fn new(id: Option<String>, name: Option<String>) -> Self {
        Self { id, name }
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn as_chunk(&self, text: String, finished: bool) -> StreamChunk {
        StreamChunk {
            text,
            finished,
            chat_id: self.id.clone(),
            chat_name: self.name.clone(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct ChatCompletionResponse {
    #[serde(default)]
    messages: Vec<ChatMessage>,
    #[serde(default)]
    choices: Vec<TiDbStreamChoice>,
    #[serde(default, alias = "conversationId", alias = "conversation_id")]
    conversation_id: Option<String>,
    #[serde(default, alias = "conversationName", alias = "conversation_name")]
    conversation_name: Option<String>,
    #[serde(default, alias = "chatId", alias = "chat_id")]
    chat_id: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    error: Option<TiDbStreamError>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct TiDbStreamChoice {
    #[serde(default)]
    message: Option<ChatMessage>,
    #[serde(default, alias = "finishReason", alias = "finish_reason")]
    finish_reason: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct TiDbStreamError {
    #[serde(default)]
    message: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct TiDbChatEnvelope {
    #[serde(default)]
    chat: Option<TiDbChatInfo>,
    #[serde(default, alias = "assistantMessage", alias = "assistant_message")]
    assistant_message: Option<TiDbAssistantMessage>,
}

#[derive(Debug, serde::Deserialize)]
struct TiDbChatInfo {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    title: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct TiDbAssistantMessage {
    #[serde(default, alias = "finishedAt", alias = "finished_at")]
    finished_at: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct TiDbStateEnvelope {
    #[serde(default)]
    state: Option<String>,
}

fn parse_chat_payload(raw_body: &str) -> Result<ChatCompletionResponse, AutoflowError> {
    serde_json::from_str(raw_body).map_err(|err| {
        AutoflowError::InvalidResponse(format!(
            "failed to parse chat response: {err}; body={}",
            truncate_debug(raw_body)
        ))
    })
}

fn to_chat_response(
    payload: ChatCompletionResponse,
    requested_chat_id: Option<String>,
) -> Result<ChatResponse, AutoflowError> {
    if let Some(err) = payload.error.as_ref().and_then(|e| e.message.clone()) {
        return Err(AutoflowError::InvalidResponse(err));
    }

    let text = extract_assistant_text(&payload)
        .ok_or_else(|| AutoflowError::InvalidResponse("empty response text".to_string()))?;

    let finished = payload.choices.is_empty()
        || payload
            .choices
            .iter()
            .any(|choice| choice.finish_reason.is_some());

    let chat_id = payload
        .chat_id
        .clone()
        .or(payload.conversation_id.clone())
        .or(requested_chat_id);

    Ok(ChatResponse {
        text,
        finished,
        chat_id,
        chat_name: payload.conversation_name.clone(),
    })
}

fn extract_assistant_text(payload: &ChatCompletionResponse) -> Option<String> {
    if let Some(content) = payload.content.as_ref() {
        if !content.trim().is_empty() {
            return Some(content.clone());
        }
    }

    if let Some(message) = payload
        .messages
        .iter()
        .rev()
        .find(|message| message.role == "assistant")
    {
        return Some(message.content.clone());
    }

    for choice in &payload.choices {
        if let Some(message) = &choice.message {
            if message.role == "assistant" {
                return Some(message.content.clone());
            }
        }
    }

    None
}
