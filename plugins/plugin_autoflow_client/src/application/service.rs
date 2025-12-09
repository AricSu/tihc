use crate::domain::{
    AutoflowConfig, AutoflowError, ChatStream, chat::AutoflowPort, session::SessionRepository,
};
use crate::infrastructure::{InMemorySessionRepository, TiDBAIClient};
use std::sync::Arc;

pub struct AutoflowSessionService {
    autoflow_port: Arc<dyn AutoflowPort>,
    session_repo: Arc<dyn SessionRepository>,
}

impl AutoflowSessionService {
    pub async fn create_session_id(&self) -> String {
        self.session_repo.create_session().await.session_id
    }
    pub fn new(config: AutoflowConfig) -> Self {
        let tidb_ai_client = Arc::new(TiDBAIClient::new(config));
        let session_repository = Arc::new(InMemorySessionRepository::new());
        Self {
            autoflow_port: tidb_ai_client as Arc<dyn AutoflowPort>,
            session_repo: session_repository as Arc<dyn SessionRepository>,
        }
    }

    // === 聊天功能方法 ===

    /// 通过 session_id 发送消息（流式）
    /// 如果 session 不存在，会自动创建一个新的 session
    pub async fn send_message(
        &self,
        session_id: Option<&str>,
        message: &str,
    ) -> Result<ChatStream, AutoflowError> {
        let session = match session_id {
            Some(id) => match self.session_repo.get(id).await {
                Some(session) => session,
                None => self.session_repo.create_session().await,
            },
            None => self.session_repo.create_session().await,
        };

        let request =
            crate::domain::chat::ChatRequest::new(vec![crate::domain::chat::ChatMessage::user(
                message,
            )])
            .with_chat_id(session.chat_id)
            .streaming(true);

        self.autoflow_port.chat_stream(request).await
    }

    pub async fn delete_chat(&self, chat_id: &str) -> Result<(), AutoflowError> {
        self.autoflow_port.delete_chat(chat_id).await?;
        self.session_repo.remove_by_chat_id(chat_id).await;

        Ok(())
    }
}
