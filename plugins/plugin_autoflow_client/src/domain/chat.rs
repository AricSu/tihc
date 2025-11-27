use std::pin::Pin;

use futures::Stream;
use serde::{Deserialize, Serialize};

use crate::AutoflowConfig;

use super::AutoflowError;

pub type ChatStream = Pin<Box<dyn Stream<Item = Result<StreamChunk, AutoflowError>> + Send>>;

use async_trait::async_trait;

#[async_trait]
pub trait AutoflowPort: Send + Sync {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AutoflowError>;
    async fn chat_stream(&self, request: ChatRequest) -> Result<ChatStream, AutoflowError>;
    async fn delete_chat(&self, chat_id: &str) -> Result<(), AutoflowError>;
    async fn health_check(&self) -> Result<bool, AutoflowError>;
    async fn update_config(&self, new_config: AutoflowConfig) -> Result<(), AutoflowError>;
    async fn get_config(&self) -> AutoflowConfig;
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn user<T: Into<String>>(content: T) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    #[serde(rename = "chat_engine", skip_serializing_if = "Option::is_none")]
    pub chat_engine: Option<String>,
    #[serde(rename = "chat_id", skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(rename = "chat_name", skip_serializing_if = "Option::is_none")]
    pub chat_name: Option<String>,
    #[serde(default)]
    pub stream: bool,
}

impl ChatRequest {
    pub fn new(messages: Vec<ChatMessage>) -> Self {
        Self {
            messages,
            chat_engine: None,
            chat_id: None,
            chat_name: None,
            stream: false,
        }
    }

    pub fn with_chat_id(mut self, chat_id: Option<String>) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn streaming(mut self, enable: bool) -> Self {
        self.stream = enable;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub text: String,
    pub finished: bool,
    pub chat_id: Option<String>,
    pub chat_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub text: String,
    pub finished: bool,
    pub chat_id: Option<String>,
    pub chat_name: Option<String>,
}
