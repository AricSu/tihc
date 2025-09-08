// Slowlog Application Service

use crate::domain::shared::DomainResult;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait SlowlogApplicationService: Send + Sync {
    async fn scan_files(&self, log_dir: &str, pattern: &str) -> DomainResult<Value>;
    async fn process_slowlog(&self, connection_id: u64, log_dir: &str, pattern: &str) -> DomainResult<Value>;
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
        use microkernel::platform::message_bus::{BusMessage, MessageBus, GLOBAL_MESSAGE_BUS};
        
        tracing::info!(target: "slowlog_service", "Scanning files in dir: {}, pattern: {}", log_dir, pattern);
        
        let args = vec![log_dir.to_string(), pattern.to_string()];
        let bus_msg = BusMessage::ok("slowlog-scan", serde_json::json!({"args": args}));
        
        let bus_result = GLOBAL_MESSAGE_BUS.send(bus_msg).await.map_err(|e| {
            crate::domain::shared::DomainError::ExternalServiceError { 
                service: format!("message_bus: {}", e) 
            }
        })?;
        
        let result = bus_result.get(0)
            .and_then(|msg| msg.data.data.clone())
            .unwrap_or(Value::Null);
            
        Ok(result)
    }
    
    async fn process_slowlog(&self, connection_id: u64, log_dir: &str, pattern: &str) -> DomainResult<Value> {
        use microkernel::platform::message_bus::{BusMessage, MessageBus, GLOBAL_MESSAGE_BUS};
        
        tracing::info!(target: "slowlog_service", "Processing slowlog for connection: {}, dir: {}, pattern: {}", 
                      connection_id, log_dir, pattern);
        
        // 1. 获取连接信息
        let conn_args = vec![connection_id.to_string()];
        let conn_bus_msg = BusMessage::ok("editor-connections-get", serde_json::json!({"args": conn_args}));
        let conn_info_result = GLOBAL_MESSAGE_BUS.send(conn_bus_msg).await.map_err(|e| {
            crate::domain::shared::DomainError::ExternalServiceError { 
                service: format!("connection_service: {}", e) 
            }
        })?;
        
        let mut connection_info = conn_info_result.get(0)
            .and_then(|msg| msg.data.data.clone())
            .unwrap_or(Value::Null);
            
        // 2. 强制替换 database 字段为 "tihc"
        if let Some(obj) = connection_info.as_object_mut() {
            obj.insert("database".to_string(), serde_json::Value::String("tihc".to_string()));
        }
        
        // 3. 处理 slowlog
        let conn_json = serde_json::to_string(&connection_info).unwrap_or_default();
        let args = vec![log_dir.to_string(), pattern.to_string(), conn_json];
        
        let import_bus_msg = BusMessage::ok("slowlog-import", serde_json::json!({"args": args}));
        let import_result = GLOBAL_MESSAGE_BUS.send(import_bus_msg).await.map_err(|e| {
            crate::domain::shared::DomainError::ExternalServiceError { 
                service: format!("slowlog_import: {}", e) 
            }
        })?;
        
        let result = import_result.get(0)
            .and_then(|msg| msg.data.data.clone())
            .unwrap_or(Value::Null);
            
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
