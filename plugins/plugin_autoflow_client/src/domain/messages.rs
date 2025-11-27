use serde::{Deserialize, Serialize};

/// AI 聊天请求消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatRequest {
    pub session_id: String,
    pub message: String,
    pub user_id: Option<i64>,
    pub chat_id: Option<String>,
}

/// AI 聊天响应消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatResponse {
    pub session_id: String,
    pub chat_id: Option<String>,
    pub text: String,
    pub finished: bool,
    pub error: Option<String>,
}

/// AI 聊天流式响应块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatChunk {
    pub session_id: Option<String>,
    pub chat_id: Option<String>,
    pub text: String,
    pub finished: bool,
}

/// 健康检查请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckRequest {}

/// 健康检查响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub message: Option<String>,
}