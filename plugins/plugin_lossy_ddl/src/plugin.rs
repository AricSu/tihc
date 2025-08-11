//! Plugin trait implementation and registration for Lossy DDL Detection.

use microkernel::plugin_api::traits::{Plugin, PluginContext};
use std::sync::Arc;
use tracing::info;

use crate::{precheck_sql_with_collation, AnalysisResult};

/// Handler for DDL analysis commands
pub struct DDLAnalysisHandler;

impl DDLAnalysisHandler {
    pub fn new() -> Self {
        Self
    }
    
    /// Analyze SQL for lossy operations
    pub fn analyze_sql(&self, sql: &str, collation_enabled: bool) -> AnalysisResult {
        precheck_sql_with_collation(sql, collation_enabled)
    }
}

/// Implementation for command handling interface if needed
#[async_trait::async_trait]
impl microkernel::platform::command_registry::CommandHandler for DDLAnalysisHandler {
    async fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        // Parse arguments: expect at least SQL and optionally collation setting
        if args.is_empty() {
            return Err(anyhow::anyhow!("Missing SQL parameter"));
        }
        
        let sql = &args[0];
        let collation_enabled = args.get(1)
            .map(|v| v.parse().unwrap_or(true))
            .unwrap_or(true);
        
        let result = self.analyze_sql(sql, collation_enabled);
        Ok(serde_json::to_value(result)?)
    }
}

/// LossyDDLPlugin for detecting lossy DDL operations
pub struct LossyDDLPlugin {
    handler: Arc<DDLAnalysisHandler>,
}

impl LossyDDLPlugin {
    /// Create a new LossyDDLPlugin instance
    pub fn new() -> Self {
        Self {
            handler: Arc::new(DDLAnalysisHandler::new()),
        }
    }
    
    /// Start background task, subscribe to shutdown signal
    pub fn start_background_task(shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        tokio::spawn(async move {
            let mut shutdown_rx = shutdown_rx;
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        info!("[LossyDDLPlugin] Background task received shutdown signal, exiting.");
                        break;
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(30)) => {
                        // Background maintenance task (if needed)
                        // For example: cleanup, health check, etc.
                        info!("[LossyDDLPlugin] Background maintenance check");
                    }
                }
            }
            info!("[LossyDDLPlugin] Background task cleanup done.");
        });
    }
}

impl Default for LossyDDLPlugin {
    fn default() -> Self {
        Self::new()
    }
}

/// Implements the Plugin trait for LossyDDLPlugin
impl Plugin for LossyDDLPlugin {
    /// Returns the plugin name for registration and discovery
    fn name(&self) -> &str {
        "lossy_ddl"
    }
    
    /// Registers all command handlers and services for the Lossy DDL plugin
    fn register(&mut self, ctx: &mut PluginContext) {
        if let Some(reg) = ctx.command_registry.as_mut() {
            // Register only the ddl-precheck command
            reg.register(
                "ddl-precheck", 
                Box::new(DDLAnalysisHandler::new()),
            );
        }
        
        // Start background task if shutdown signal is provided by the platform
        if let Some(shutdown_rx) = ctx.shutdown_rx.take() {
            LossyDDLPlugin::start_background_task(shutdown_rx);
        }
        
        info!("[LossyDDLPlugin] Plugin registered successfully with ddl-precheck command");
    }
}
