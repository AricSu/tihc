// Slowlog Application Service

use crate::domain::shared::DomainResult;
use async_trait::async_trait;
use microkernel::platform::message_bus::{BusMessage, GLOBAL_MESSAGE_BUS};
use microkernel::topic;
use serde_json::from_value;
use serde_json::Value;
#[async_trait]
pub trait SlowlogApplicationService: Send + Sync {
    async fn scan_files(&self, log_dir: &str, pattern: &str) -> DomainResult<Value>;
    async fn process_slowlog(
        &self,
        connection_id: u64,
        log_dir: &str,
        pattern: &str,
    ) -> DomainResult<Value>;
    async fn get_progress(&self, job_id: &str) -> DomainResult<Value>;
}

pub struct SlowlogApplicationServiceImpl {
    // 这里可以注入消息总线或其他依赖
}

impl SlowlogApplicationServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SlowlogApplicationService for SlowlogApplicationServiceImpl {
    async fn scan_files(&self, log_dir: &str, pattern: &str) -> DomainResult<Value> {
        use microkernel::platform::message_bus::{BusMessage, GLOBAL_MESSAGE_BUS};
        use serde_json::from_value;

        tracing::info!(target: "slowlog_service", "Scanning files in dir: {}, pattern: {}", log_dir, pattern);

        let args = vec![log_dir.to_string(), pattern.to_string()];
        let topic = topic!("slowlog-scan");
        let bus_msg = BusMessage::ok(topic, serde_json::json!({"args": args}));
        let reply = GLOBAL_MESSAGE_BUS
            .request(bus_msg, None)
            .await
            .map_err(
                |e| crate::domain::shared::DomainError::ExternalServiceError {
                    service: format!("message_bus: {}", e),
                },
            )?;
        let result: Vec<Value> = from_value(reply.data).map_err(|e| {
            crate::domain::shared::DomainError::ExternalServiceError {
                service: format!("serde_json error: {}", e),
            }
        })?;
        let result = result.get(0).cloned().unwrap_or(Value::Null);
        Ok(result)
    }

    async fn process_slowlog(
        &self,
        connection_id: u64,
        log_dir: &str,
        pattern: &str,
    ) -> DomainResult<Value> {
        tracing::info!(target: "slowlog_service", "Processing slowlog for connection: {}, dir: {}, pattern: {}", 
                      connection_id, log_dir, pattern);

        // 1. 获取连接信息
        let conn_args = vec![connection_id.to_string()];
        let topic = topic!("editor-connections-get");
        let conn_bus_msg = BusMessage::ok(topic, serde_json::json!({"args": conn_args}));
        let reply = GLOBAL_MESSAGE_BUS
            .request(conn_bus_msg, None)
            .await
            .map_err(
                |e| crate::domain::shared::DomainError::ExternalServiceError {
                    service: format!("connection_service: {}", e),
                },
            )?;
        let mut conn_vec: Vec<Value> = from_value(reply.data).map_err(|e| {
            crate::domain::shared::DomainError::ExternalServiceError {
                service: format!("serde_json error: {}", e),
            }
        })?;
        let mut connection_info = conn_vec.get(0).cloned().unwrap_or(Value::Null);

        // 2. 强制替换 database 字段为 "tihc"
        if let Some(obj) = connection_info.as_object_mut() {
            obj.insert(
                "database".to_string(),
                serde_json::Value::String("tihc".to_string()),
            );
        }

        // 3. 处理 slowlog
        let conn_json = serde_json::to_string(&connection_info).unwrap_or_default();
        let args = vec![log_dir.to_string(), pattern.to_string(), conn_json];

        let topic = topic!("slowlog-import");
        let import_bus_msg = BusMessage::ok(topic, serde_json::json!({"args": args}));
        let reply = GLOBAL_MESSAGE_BUS
            .request(import_bus_msg, None)
            .await
            .map_err(
                |e| crate::domain::shared::DomainError::ExternalServiceError {
                    service: format!("slowlog_import: {}", e),
                },
            )?;
        let import_vec: Vec<Value> = from_value(reply.data).map_err(|e| {
            crate::domain::shared::DomainError::ExternalServiceError {
                service: format!("serde_json error: {}", e),
            }
        })?;
        let result = import_vec.get(0).cloned().unwrap_or(Value::Null);
        Ok(result)
    }

    async fn get_progress(&self, job_id: &str) -> DomainResult<Value> {
        // TODO: 实现真正的进度查询逻辑
        Ok(serde_json::json!({
            "job_id": job_id,
            "progress": 0.5,
            "status": "running"
        }))
    }
}
