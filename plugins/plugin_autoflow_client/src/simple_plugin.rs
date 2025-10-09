use crate::simple_client::{
    AutoflowClient, AutoflowConfig, AutoflowError, ChatResponse, ChatStream,
};
use std::sync::Arc;

/// 简化的插件结构
pub struct AutoflowPlugin {
    client: Arc<AutoflowClient>,
}

impl AutoflowPlugin {
    pub fn new(config: AutoflowConfig) -> Self {
        let client = Arc::new(AutoflowClient::new(config));
        Self { client }
    }

    /// 发送消息
    pub async fn send_message(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatResponse, AutoflowError> {
        self.client.send_message(session_id, message).await
    }

    /// 发送流式消息
    pub async fn send_message_stream(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatStream, AutoflowError> {
        self.client.send_message_stream(session_id, message).await
    }

    /// 创建会话
    pub async fn create_session(&self) -> Result<String, AutoflowError> {
        self.client.create_session().await
    }

    /// 健康检查
    pub async fn health_check(&self) -> Result<bool, AutoflowError> {
        self.client.health_check().await
    }
}

/// 插件工厂函数
pub fn create_autoflow_plugin(config: AutoflowConfig) -> AutoflowPlugin {
    AutoflowPlugin::new(config)
}
