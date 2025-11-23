use crate::domain::auth::AuthTokenStore;
use crate::infrastructure::auth::di::{
    ConcreteAuthService, ConcreteOAuthService, ConcreteUserService, DiContainer,
};
use crate::infrastructure::repositories::{ChatHistoryRepository, MySqlMenuRepository};
use std::sync::Arc;

/// 只负责基础设施对象，不 new 任何 Service
/// InfraState 从 DiContainer 中获取所有需要的组件
pub struct InfraState {
    pub auth_service: Arc<ConcreteAuthService>,
    pub oauth_service: Arc<ConcreteOAuthService>,
    pub user_service: Arc<ConcreteUserService>,
    pub menu_repo: Arc<MySqlMenuRepository>,
    pub chat_history_repo: Arc<ChatHistoryRepository>,
    pub auth_token_store: Arc<dyn AuthTokenStore>,
    pub ai_service: Arc<crate::application::ai::AiService>,
}

impl InfraState {
    pub fn from_di_container(container: DiContainer) -> Self {
        Self {
            auth_service: container.auth_service,
            oauth_service: container.oauth_service,
            user_service: container.user_service,
            menu_repo: container.menu_repo,
            chat_history_repo: container.chat_history_repo,
            auth_token_store: container.auth_token_store,
            ai_service: Arc::new(crate::application::ai::AiService::new()),
        }
    }
}

pub async fn create_infra_state(config_value: &toml::Value) -> anyhow::Result<InfraState> {
    let container = DiContainer::new(config_value).await?;
    Ok(InfraState::from_di_container(container))
}
