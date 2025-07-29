use crate::application::slowlog_service::SlowLogService;
use crate::domain::ImportStatus;
use crate::domain::SlowLogScanResult;
use core::platform::command_registry::CommandHandler;
use core::platform::service_registry::ServiceRegistry;
use serde_json;
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct SlowLogCommandHandler {
    pub registry: Arc<Mutex<ServiceRegistry>>,
}

impl CommandHandler for SlowLogCommandHandler {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        let log_dir = args.get(0).cloned().unwrap_or_else(|| ".".to_string());
        let pattern = args.get(1).cloned().unwrap_or_else(|| "*.log".to_string());
        let registry_guard = self.registry.lock().unwrap();
        let service = registry_guard
            .resolve::<Box<dyn SlowLogService>>()
            .ok_or_else(|| anyhow::anyhow!("SlowLogService not found"))?;
        let rt = tokio::runtime::Runtime::new()?;
        let files = rt.block_on(service.scan_files(&log_dir, &pattern))?;
        info!("Matched files: {:?}", files);
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
        let log_dir = args.get(0).cloned().unwrap_or_else(|| ".".to_string());
        let pattern = args.get(1).cloned().unwrap_or_else(|| "*.log".to_string());
        let registry_guard = self.registry.lock().unwrap();
        let service = registry_guard
            .resolve::<Box<dyn SlowLogService>>()
            .ok_or_else(|| anyhow::anyhow!("SlowLogService not found"))?;
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(service.parse_and_import(&log_dir, &pattern))?;
        let status = ImportStatus {
            status: "imported".to_string(),
        };
        Ok(serde_json::to_value(status)?)
    }
}
