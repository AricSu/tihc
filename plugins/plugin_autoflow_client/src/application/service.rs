use std::sync::Arc;
use crate::domain::{
    AutoflowConfig, AutoflowError, ChatStream,
    chat::AutoflowPort,
    session::SessionRepository,
};
use crate::infrastructure::{TiDBAIClient, InMemorySessionRepository};

/// 统一的 AutoflowSessionService - 应用层服务
/// 负责协调聊天功能和会话管理
pub struct AutoflowSessionService {
    autoflow_port: Arc<dyn AutoflowPort>,
    session_repo: Arc<dyn SessionRepository>,
}

impl AutoflowSessionService {
        /// 外部创建新 session 并返回 session_id
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
            Some(id) => {
                match self.session_repo.get(id).await {
                    Some(session) => session,
                    None => self.session_repo.create_session().await
                }
            }
            None => {
                self.session_repo.create_session().await
            }
        };

         let request = crate::domain::chat::ChatRequest::new(vec![
            crate::domain::chat::ChatMessage::user(message)
        ])
        .with_chat_id(session.chat_id)
        .streaming(true);
        
        self.autoflow_port.chat_stream(request).await
    }

    pub async fn delete_chat(&self, chat_id: &str) -> Result<(), AutoflowError> {
        // 删除聊天
        self.autoflow_port.delete_chat(chat_id).await?;
        
        // 删除相关会话
        self.session_repo.remove_by_chat_id(chat_id).await;
        
        Ok(())
    }

    // === 配置和健康检查方法 ===

    pub async fn health_check(&self) -> Result<HealthStatus, AutoflowError> {
        let healthy = self.autoflow_port.health_check().await?;

        Ok(HealthStatus {
            status: if healthy { "healthy" } else { "unhealthy" }.to_string(),
            timestamp: chrono::Utc::now(),
            checks: vec![HealthCheck {
                name: "autoflow_api".to_string(),
                status: if healthy { "up" } else { "down" }.to_string(),
                message: None,
            }],
        })
    }

    pub async fn update_config(&self, new_config: AutoflowConfig) -> Result<(), AutoflowError> {
        self.autoflow_port.update_config(new_config).await
    }

    pub async fn get_config(&self) -> AutoflowConfig {
        self.autoflow_port.get_config().await
    }
}

// 健康检查相关类型
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}