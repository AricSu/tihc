use crate::infrastructure::auth::di::{
    ConcreteAuthService, ConcreteOAuthService, ConcreteUserService, DiContainer,
};
use crate::infrastructure::repositories::{ChatHistoryRepository, MySqlMenuRepository};
use std::sync::Arc;
use microkernel::event_bus::EventBus;
use microkernel::plugin::PluginEvent;

/// 只负责基础设施对象，不 new 任何 Service
/// InfraState 从 DiContainer 中获取所有需要的组件
pub struct InfraState {
    pub auth_service: Arc<ConcreteAuthService>,
    pub oauth_service: Arc<ConcreteOAuthService>,
    pub user_service: Arc<ConcreteUserService>,
    pub menu_repo: Arc<MySqlMenuRepository>,
    pub chat_history_repo: Arc<ChatHistoryRepository>,
    pub ai_service: Arc<crate::application::ai::AiService>,
    pub token_service: Arc<crate::application::auth::token_service::TokenService<crate::infrastructure::repositories::token::MySqlTokenRepository>>,
}

impl InfraState {
    pub fn from_di_container(container: DiContainer, bus: Arc<EventBus<PluginEvent>>) -> Self {
        Self {
            auth_service: container.auth_service,
            oauth_service: container.oauth_service,
            user_service: container.user_service,
            menu_repo: container.menu_repo,
            chat_history_repo: container.chat_history_repo,
            ai_service: Arc::new(crate::application::ai::AiService::new(bus)),
            token_service: container.token_service,
        }
    }
}

pub async fn create_infra_state(config_value: &toml::Value, bus: Arc<EventBus<PluginEvent>>) -> anyhow::Result<InfraState> {
    let container = DiContainer::new(config_value).await?;
    Ok(InfraState::from_di_container(container, bus))
}
