use crate::application::slowlog_service::SlowLogService;
use crate::domain::ImportStatus;
use crate::domain::SlowLogScanResult;
use microkernel::platform::command_registry::CommandHandler;
use microkernel::platform::service_registry::ServiceRegistry;
use serde_json;
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct SlowLogScanHandler {
    pub registry: Arc<Mutex<ServiceRegistry>>,
}

impl CommandHandler for SlowLogScanHandler {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        // 支持前端动态传递 conn 参数（如 json 字符串或各字段）
        let log_dir = args
            .get(0)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Missing log_dir argument"))?;
        let pattern = args
            .get(1)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Missing pattern argument"))?;
        let conn_json = args.get(2).cloned().unwrap_or_default();
        let conn: crate::domain::connection::Connection = if !conn_json.is_empty() {
            serde_json::from_str(&conn_json)?
        } else {
            crate::domain::connection::Connection {
                id: 0,
                name: "default".to_string(),
                engine: "tidb".to_string(),
                host: "localhost".to_string(),
                port: 4000,
                username: "root".to_string(),
                password: None,
                database: None,
                use_tls: false,
                ca_cert_path: None,
            }
        };
        let service = crate::application::slowlog_service::SlowLogServiceImpl::new(128, conn);
        let files = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(service.scan_files(&log_dir, &pattern))
        })?;
        let result = SlowLogScanResult {
            matched_files: files,
        };
        Ok(serde_json::to_value(result)?)
    }
}

pub struct SlowLogParseAndImportHandler {
    pub registry: Arc<Mutex<ServiceRegistry>>,
}

impl CommandHandler for SlowLogParseAndImportHandler {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        let log_dir = args
            .get(0)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Missing log_dir argument"))?;
        let pattern = args
            .get(1)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Missing pattern argument"))?;
        let conn_json = args.get(2).cloned().unwrap_or_default();
        let conn: crate::domain::connection::Connection = if !conn_json.is_empty() {
            serde_json::from_str(&conn_json)?
        } else {
            crate::domain::connection::Connection {
                id: 0,
                name: "default".to_string(),
                engine: "tidb".to_string(),
                host: "localhost".to_string(),
                port: 4000,
                username: "root".to_string(),
                password: None,
                database: None,
                use_tls: false,
                ca_cert_path: None,
            }
        };
        let service = crate::application::slowlog_service::SlowLogServiceImpl::new(128, conn);
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(service.parse_and_import(&log_dir, &pattern))
        })?;
        let status = ImportStatus {
            status: "imported".to_string(),
        };
        Ok(serde_json::to_value(status)?)
    }
}
