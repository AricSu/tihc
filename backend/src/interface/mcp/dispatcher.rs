use serde_json::Value;
use std::time::Duration;
use thiserror::Error;
use microkernel::platform::message_bus::{GLOBAL_MESSAGE_BUS, MessageBus};
use crate::interface::mcp::registry::ComponentRegistry;

#[derive(Error, Debug)]
pub enum DispatchError {
    #[error("Component not found for method: {0}")]
    ComponentNotFound(String),
    #[error("Message bus error: {0}")]
    MessageBusError(String),
    #[error("Timeout waiting for response")]
    Timeout,
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
}

/// MCP 请求分发器 - 桥接 MCP 协议和消息总线
/// 
/// 职责：
/// 1. 接收 MCP 方法调用请求
/// 2. 根据方法名路由到对应的消息总线主题
/// 3. 处理超时和错误情况
/// 4. 将消息总线响应转换回 MCP 格式
#[derive(Clone)]
pub struct RequestDispatcher {
    registry: ComponentRegistry,
    timeout: Duration,
}

impl RequestDispatcher {
    pub fn new(registry: ComponentRegistry) -> Self {
        Self {
            registry,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 分发 MCP 请求到消息总线
    /// - method: MCP 方法名 (list_tools, call_tool, etc.)
    /// - params: MCP 请求参数 (已经是预处理过的 JSON)
    pub async fn dispatch(&self, method: &str, params: Value) -> Result<Value, DispatchError> {
        // 1. 查找负责处理该方法的组件主题
        let topic = self.registry.find_topic_for_method(method)
            .ok_or_else(|| DispatchError::ComponentNotFound(method.to_string()))?;

        // 2. 通过消息总线发送请求并等待响应
        match GLOBAL_MESSAGE_BUS.send_with_timeout(&topic, params, self.timeout).await {
            Ok(response) => Ok(response),
            Err(e) => {
                if e.to_string().contains("timeout") {
                    Err(DispatchError::Timeout)
                } else {
                    Err(DispatchError::MessageBusError(e.to_string()))
                }
            }
        }
    }

    /// 广播初始化消息到所有组件
    pub async fn broadcast_initialize(&self, params: Value) -> Result<Vec<Value>, DispatchError> {
        let topics = self.registry.get_all_topics();
        let mut results = Vec::new();

        for topic in topics {
            match GLOBAL_MESSAGE_BUS.send_with_timeout(&topic, params.clone(), self.timeout).await {
                Ok(response) => results.push(response),
                Err(e) => {
                    tracing::warn!("Failed to initialize component {}: {}", topic, e);
                    // 继续处理其他组件，不因为单个组件失败而中断
                }
            }
        }

        Ok(results)
    }

    /// 健康检查 - 检查所有注册组件的状态
    pub async fn health_check(&self) -> Result<Value, DispatchError> {
        let topics = self.registry.get_all_topics();
        let mut health_status = serde_json::Map::new();

        let ping_params = serde_json::json!({
            "method": "health_check",
            "timestamp": chrono::Utc::now().timestamp_millis()
        });

        for topic in topics {
            let start_time = std::time::Instant::now();
            match GLOBAL_MESSAGE_BUS.send_with_timeout(&topic, ping_params.clone(), Duration::from_secs(5)).await {
                Ok(_) => {
                    let response_time = start_time.elapsed().as_millis();
                    health_status.insert(topic, serde_json::json!({
                        "status": "healthy",
                        "response_time_ms": response_time
                    }));
                }
                Err(e) => {
                    health_status.insert(topic, serde_json::json!({
                        "status": "unhealthy",
                        "error": e.to_string(),
                        "response_time_ms": start_time.elapsed().as_millis()
                    }));
                }
            }
        }

        Ok(serde_json::json!({
            "overall_status": if health_status.values().all(|v| v["status"] == "healthy") {
                "healthy"
            } else {
                "degraded"
            },
            "components": health_status,
            "checked_at": chrono::Utc::now().to_rfc3339()
        }))
    }

    /// 获取分发器统计信息
    pub fn get_stats(&self) -> Value {
        serde_json::json!({
            "registered_components": self.registry.get_all_topics().len(),
            "timeout_seconds": self.timeout.as_secs(),
            "registry_info": self.registry.get_component_info()
        })
    }
}
