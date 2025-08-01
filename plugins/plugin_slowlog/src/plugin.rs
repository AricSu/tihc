use crate::application::handler::{SlowLogParseAndImportHandler, SlowLogScanHandler};
use microkernel::plugin_api::traits::{Plugin, PluginContext};
use std::sync::Arc;
use tracing::info;

/// SlowLogPlugin 支持优雅关闭
pub struct SlowLogPlugin {
    // 不再持有后台任务句柄，完全解耦任务管理
}

impl SlowLogPlugin {
    /// 构造函数，初始化关闭标志
    pub fn new() -> Self {
        Self {}
    }
    /// 启动后台任务，订阅 shutdown 信号
    pub fn start_background_task(shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        tokio::spawn(async move {
            let mut shutdown_rx = shutdown_rx;
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        info!("[SlowLogPlugin] Background task received shutdown signal, exiting.");
                        break;
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                        // ...实际后台任务逻辑...
                    }
                }
            }
            info!("[SlowLogPlugin] Background task cleanup done.");
        });
    }
}

impl Plugin for SlowLogPlugin {
    fn name(&self) -> &str {
        "slowlog"
    }
    fn register(&mut self, ctx: &mut PluginContext) {
        if let Some(reg) = ctx.command_registry.as_mut() {
            reg.register(
                "slowlog-scan",
                Box::new(SlowLogScanHandler {
                    registry: Arc::clone(&ctx.service_registry),
                }),
            );
            reg.register(
                "slowlog-import",
                Box::new(SlowLogParseAndImportHandler {
                    registry: Arc::clone(&ctx.service_registry),
                }),
            );
        }
        // 后台任务由平台统一调度，传入 shutdown_rx
        if let Some(shutdown_rx) = ctx.shutdown_rx.take() {
            SlowLogPlugin::start_background_task(shutdown_rx);
        }
    }
}
