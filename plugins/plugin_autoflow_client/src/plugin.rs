use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::{AutoflowHttpClient, AutoflowConfig, SessionManager, AutoflowError, ChatStream, StreamChunk};

/// Autoflow 客户端插件主体
pub struct AutoflowClientPlugin {
    client: Arc<AutoflowHttpClient>,
    session_manager: Arc<SessionManager>,
    config: Arc<RwLock<AutoflowConfig>>,
}

impl AutoflowClientPlugin {
    pub fn new(config: AutoflowConfig) -> Self {
        let client = Arc::new(AutoflowHttpClient::new(config.clone()));
        let session_manager = Arc::new(SessionManager::new());
        
        Self {
            client,
            session_manager,
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// 发送同步消息
    pub async fn send_message(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<crate::client::ChatResponse, AutoflowError> {
        // 验证会话是否存在
        if self.session_manager.get_session(session_id).await.is_none() {
            return Err(AutoflowError::SessionNotFound(session_id.to_string()));
        }

        self.client.send_message(session_id, message).await
    }

    /// 发送流式消息 (SSE)
    pub async fn send_message_stream_sse(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatStream, AutoflowError> {
        // 验证会话是否存在
        if self.session_manager.get_session(session_id).await.is_none() {
            return Err(AutoflowError::SessionNotFound(session_id.to_string()));
        }

        self.client.send_message_stream_sse(session_id, message).await
    }

    /// 发送流式消息 (HTTP Stream)
    pub async fn send_message_stream_http(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatStream, AutoflowError> {
        // 验证会话是否存在
        if self.session_manager.get_session(session_id).await.is_none() {
            return Err(AutoflowError::SessionNotFound(session_id.to_string()));
        }

        self.client.send_message_stream_http(session_id, message).await
    }

    /// 创建会话
    pub async fn create_session(&self) -> Result<String, AutoflowError> {
        let session_id = self.session_manager.create_session().await;
        Ok(session_id)
    }

    /// 关闭会话
    pub async fn close_session(&self, session_id: &str) -> Result<(), AutoflowError> {
        if !self.session_manager.close_session(session_id).await {
            return Err(AutoflowError::SessionNotFound(session_id.to_string()));
        }
        
        self.client.close_session(session_id).await
    }

    /// 获取会话列表
    pub async fn list_sessions(&self) -> Vec<String> {
        // 实现获取会话列表逻辑
        vec![]
    }

    /// 健康检查
    pub async fn health_check(&self) -> Result<HealthStatus, AutoflowError> {
        let is_healthy = self.client.health_check().await.unwrap_or(false);
        
        Ok(HealthStatus {
            status: if is_healthy { "healthy".to_string() } else { "unhealthy".to_string() },
            timestamp: chrono::Utc::now(),
            checks: vec![
                HealthCheck {
                    name: "autoflow_api".to_string(),
                    status: if is_healthy { "up".to_string() } else { "down".to_string() },
                    message: None,
                }
            ],
        })
    }

    /// 更新配置
    pub async fn update_config(&self, new_config: AutoflowConfig) -> Result<(), AutoflowError> {
        self.client.update_config(new_config.clone()).await;
        
        let mut config = self.config.write().await;
        *config = new_config;
        
        Ok(())
    }

    /// 清理过期会话
    pub async fn cleanup_expired_sessions(&self) -> Result<usize, AutoflowError> {
        let config = self.config.read().await;
        let ttl_minutes = 60; // 1小时过期
        let cleaned = self.session_manager.cleanup_expired_sessions(ttl_minutes).await;
        Ok(cleaned)
    }
}

// 健康检查相关结构体
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

// 实现 Plugin trait (需要根据具体的 microkernel 接口调整)
/*
use microkernel::plugin_api::traits::Plugin;

#[async_trait]
impl Plugin for AutoflowClientPlugin {
    fn name(&self) -> &str {
        "autoflow_client"
    }
    
    fn description(&self) -> &str {
        "TiDB.ai Autoflow client plugin for AI-powered database assistance"
    }
    
    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
    
    async fn initialize(&mut self, config: serde_json::Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 从配置中初始化插件
        if let Ok(autoflow_config) = serde_json::from_value::<AutoflowConfig>(config) {
            self.update_config(autoflow_config).await?;
        }
        
        tracing::info!("Autoflow client plugin initialized");
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 清理资源
        self.cleanup_expired_sessions().await?;
        tracing::info!("Autoflow client plugin shut down");
        Ok(())
    }
}
*/

/// 插件工厂函数
pub fn create_autoflow_plugin(config: AutoflowConfig) -> AutoflowClientPlugin {
    AutoflowClientPlugin::new(config)
}