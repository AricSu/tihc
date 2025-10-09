use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use reqwest::RequestBuilder;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use uuid::Uuid;
use tracing;

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
    
    #[error("Service unavailable")]
    ServiceUnavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoflowConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub default_engine: String,
}

impl Default for AutoflowConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.tidb.ai".to_string(),
            api_key: None,
            timeout_seconds: 30,
            max_retries: 3,
            default_engine: "gpt-4".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub session_id: String,
    pub message: String,
    pub engine: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub text: String,
    pub finished: bool,
    pub session_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamChunk {
    pub session_id: String,
    pub text: String,
    pub finished: bool,
    pub timestamp: i64,
}

// 流式响应类型
pub type ChatStream = Pin<Box<dyn Stream<Item = Result<StreamChunk, AutoflowError>> + Send>>;

/// Autoflow HTTP 客户端
pub struct AutoflowHttpClient {
    client: Client,
    config: Arc<RwLock<AutoflowConfig>>,
}

impl AutoflowHttpClient {
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
            temperature: Some(0.7),
            max_tokens: Some(4000),
            stream: false,
        };

        let url = format!("{}/chat", config.base_url);
        let mut req_builder = self.client.post(&url).json(&request);

        if let Some(api_key) = &config.api_key {
            req_builder = req_builder.bearer_auth(api_key);
        }

        let response = req_builder.send().await?;

        if !response.status().is_success() {
            return Err(AutoflowError::HttpError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    }

    /// 发送流式消息 (SSE)
    pub async fn send_message_stream_sse(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatStream, AutoflowError> {
        let config = self.config.read().await;
        let url = format!("{}/chat/stream", config.base_url);
        
        let mut req_builder = reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/json");

        if let Some(api_key) = &config.api_key {
            req_builder = req_builder.bearer_auth(api_key);
        }

        let req_builder = req_builder.json(&ChatRequest {
            session_id: session_id.to_string(),
            message: message.to_string(),
            engine: Some(config.default_engine.clone()),
            temperature: Some(0.7),
            max_tokens: Some(4000),
            stream: true,
        });

        let client = EventSource::new(req_builder)?;

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let session_id = session_id.to_string();

        // 处理 SSE 事件流
        tokio::spawn(async move {
            let mut stream = client;
            
            while let Some(event) = stream.next().await {
                match event {
                    Ok(Event::Message(msg)) => {
                        if let Ok(chunk) = serde_json::from_str::<StreamChunk>(&msg.data) {
                            if tx.send(Ok(chunk.clone())).is_err() {
                                break;
                            }
                            
                            if chunk.finished {
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

    /// 发送流式消息 (HTTP Stream)
    pub async fn send_message_stream_http(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatStream, AutoflowError> {
        let config = self.config.read().await;
        let request = ChatRequest {
            session_id: session_id.to_string(),
            message: message.to_string(),
            engine: Some(config.default_engine.clone()),
            temperature: Some(0.7),
            max_tokens: Some(4000),
            stream: true,
        };

        let url = format!("{}/chat/stream", config.base_url);
        let mut req_builder = self.client.post(&url).json(&request);

        if let Some(api_key) = &config.api_key {
            req_builder = req_builder.bearer_auth(api_key);
        }

        let response = req_builder.send().await?;

        if !response.status().is_success() {
            return Err(AutoflowError::HttpError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        // 处理流式响应
        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        if let Ok(text) = std::str::from_utf8(&chunk) {
                            buffer.push_str(text);
                            
                            // 按行分割处理 NDJSON
                            while let Some(line_end) = buffer.find('\n') {
                                let line = buffer[..line_end].trim();
                                buffer = buffer[line_end + 1..].to_string();
                                
                                if !line.is_empty() {
                                    if let Ok(stream_chunk) = serde_json::from_str::<StreamChunk>(line) {
                                        let finished = stream_chunk.finished;
                                        if tx.send(Ok(stream_chunk)).is_err() {
                                            return;
                                        }
                                        
                                        if finished {
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let error = AutoflowError::StreamError(format!("Stream error: {}", e));
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
        let session_id = Uuid::new_v4().to_string();
        // 这里可以调用实际的 API 创建会话
        Ok(session_id)
    }

    /// 关闭会话
    pub async fn close_session(&self, _session_id: &str) -> Result<(), AutoflowError> {
        // 实现会话关闭逻辑
        Ok(())
    }

    /// 更新配置
    pub async fn update_config(&self, new_config: AutoflowConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    /// 健康检查
    pub async fn health_check(&self) -> Result<bool, AutoflowError> {
        let config = self.config.read().await;
        let url = format!("{}/health", config.base_url);
        
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}

// 会话管理器
#[derive(Debug)]
pub struct SessionManager {
    sessions: Arc<RwLock<std::collections::HashMap<String, chrono::DateTime<chrono::Utc>>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn create_session(&self) -> String {
        let session_id = Uuid::new_v4().to_string();
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), chrono::Utc::now());
        session_id
    }

    pub async fn get_session(&self, session_id: &str) -> Option<chrono::DateTime<chrono::Utc>> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    pub async fn close_session(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id).is_some()
    }

    pub async fn cleanup_expired_sessions(&self, ttl_minutes: i64) -> usize {
        let mut sessions = self.sessions.write().await;
        let cutoff = chrono::Utc::now() - chrono::Duration::minutes(ttl_minutes);
        
        let expired_sessions: Vec<String> = sessions
            .iter()
            .filter(|(_, created_at)| **created_at < cutoff)
            .map(|(id, _)| id.clone())
            .collect();
        
        for session_id in &expired_sessions {
            sessions.remove(session_id);
        }
        
        expired_sessions.len()
    }
}