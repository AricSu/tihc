use std::sync::Arc;

use crate::domain::chat::{ChatHistory, ChatHistoryRequest, ChatSession};
use crate::infrastructure::repositories::ChatHistoryRepository;
use crate::application::ai::{AiService, AiChatResponse};

pub struct ChatHistoryService {
    chat_history_repository: Arc<ChatHistoryRepository>,
    ai_service: Arc<AiService>,
}

impl ChatHistoryService {
    pub fn new(
        chat_history_repository: Arc<ChatHistoryRepository>,
        ai_service: Arc<AiService>,
    ) -> Self {
        Self {
            chat_history_repository,
            ai_service,
        }
    }

    pub async fn add_chat_message(
        &self,
        user_id: i64,
        session_id: Option<i64>,
        user_message: String,
        assistant_message: String,
    ) -> anyhow::Result<ChatHistory> {
        self.chat_history_repository
            .create(user_id, session_id, user_message, assistant_message)
            .await
    }

    pub async fn get_chat_history(
        &self,
        request: ChatHistoryRequest,
    ) -> anyhow::Result<Vec<ChatHistory>> {
        let limit = request.limit.unwrap_or(10);
        if let Some(session_id) = request.session_id {
            self.chat_history_repository
                .find_by_session_id(session_id, limit)
                .await
        } else {
            self.chat_history_repository
                .find_latest_by_user_id(request.user_id, limit)
                .await
        }
    }

    pub async fn create_session(
        &self,
        user_id: i64,
        title: Option<String>,
    ) -> anyhow::Result<ChatSession> {
        self.chat_history_repository
            .create_session(user_id, title)
            .await
    }

    pub async fn list_sessions(
        &self,
        user_id: i64,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<ChatSession>> {
        self.chat_history_repository
            .list_sessions_by_user(user_id, limit)
            .await
    }

    /// 发送 AI 聊天消息并保存对话历史
    pub async fn send_ai_message(
        &self,
        user_id: i64,
        session_id: Option<i64>,
        user_message: String,
    ) -> anyhow::Result<(ChatHistory, AiChatResponse)> {
        // 生成用于 AI 服务的会话 ID
        let ai_session_id = match session_id {
            Some(id) => format!("backend_session_{}", id),
            None => format!("backend_user_{}", user_id),
        };

        // 通过消息总线发送 AI 请求
        let ai_response = self
            .ai_service
            .send_chat_message(ai_session_id, user_message.clone(), Some(user_id))
            .await?;

        // 检查是否有错误
        if let Some(error) = &ai_response.error {
            return Err(anyhow::anyhow!("AI service error: {}", error));
        }

        // 保存聊天历史
        let chat_history = self
            .chat_history_repository
            .create(user_id, session_id, user_message, ai_response.text.clone())
            .await?;

        Ok((chat_history, ai_response))
    }
}
