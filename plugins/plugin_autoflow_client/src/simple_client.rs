use futures::Stream;
use reqwest::Client;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AutoflowError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Request timeout")]
    Timeout,

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Stream error: {0}")]
    StreamError(String),

    #[error("Authentication failed")]
    AuthenticationFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoflowConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
    pub default_engine: String,
}

impl Default for AutoflowConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.tidb.ai".to_string(),
            api_key: None,
            timeout_seconds: 30,
            default_engine: "gpt-4".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub session_id: String,
    pub message: String,
    pub engine: Option<String>,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatResponse {
    pub text: String,
    pub finished: bool,
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamChunk {
    pub session_id: String,
    pub text: String,
    pub finished: bool,
}

// 流式响应类型
pub type ChatStream = Pin<Box<dyn Stream<Item = Result<StreamChunk, AutoflowError>> + Send>>;

/// 简化的 Autoflow 客户端
pub struct AutoflowClient {
    client: Client,
    config: Arc<RwLock<AutoflowConfig>>,
}

impl AutoflowClient {
    pub fn new(config: AutoflowConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// 发送同步消息
    pub async fn send_message(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatResponse, AutoflowError> {
        let config = self.config.read().await;
        let request = ChatRequest {
            session_id: session_id.to_string(),
            message: message.to_string(),
            engine: Some(config.default_engine.clone()),
            stream: false,
        };

        let url = format!("{}/chat", config.base_url);
        let mut req_builder = self.client.post(&url).json(&request);

        if let Some(api_key) = &config.api_key {
            req_builder = req_builder.bearer_auth(api_key);
        }

        let response = req_builder.send().await?;
        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    }

    /// 发送流式消息 (SSE) - 简化版本，暂时不使用认证
    pub async fn send_message_stream(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatStream, AutoflowError> {
        let config = self.config.read().await;
        let url = format!(
            "{}/chat/stream?session_id={}&message={}",
            config.base_url,
            urlencoding::encode(session_id),
            urlencoding::encode(message)
        );

        // 创建 EventSource
        let client = EventSource::get(&url);

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let session_id = session_id.to_string();

        // 处理 SSE 事件流
        tokio::spawn(async move {
            let mut stream = client;

            while let Some(event) = stream.next().await {
                match event {
                    Ok(Event::Message(msg)) => {
                        if let Ok(chunk) = serde_json::from_str::<StreamChunk>(&msg.data) {
                            let finished = chunk.finished;
                            if tx.send(Ok(chunk)).is_err() {
                                break;
                            }
                            if finished {
                                break;
                            }
                        }
                    }
                    Ok(Event::Open) => {
                        tracing::info!("SSE connection opened for session: {}", session_id);
                    }
                    Err(e) => {
                        let error = AutoflowError::StreamError(format!("SSE error: {}", e));
                        let _ = tx.send(Err(error));
                        break;
                    }
                }
            }
        });

        Ok(Box::pin(UnboundedReceiverStream::new(rx)))
    }

    /// 创建会话
    pub async fn create_session(&self) -> Result<String, AutoflowError> {
        Ok(Uuid::new_v4().to_string())
    }

    /// 健康检查
    pub async fn health_check(&self) -> Result<bool, AutoflowError> {
        let config = self.config.read().await;
        let url = format!("{}/health", config.base_url);

        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }

    /// 更新配置
    pub async fn update_config(&self, new_config: AutoflowConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }
}
