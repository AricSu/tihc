use std::sync::Arc;
use crate::application::handler::{SlowLogCommandHandler, SlowLogParseAndImportHandler};
use crate::application::slowlog_service::{SlowLogService, SlowLogServiceImpl};
use core::plugin_api::traits::{Plugin, PluginContext};
pub struct SlowLogPlugin;

impl Plugin for SlowLogPlugin {
    fn name(&self) -> &str { "slowlog" }
    fn register(&mut self, ctx: &mut PluginContext) {
        // 只注册 trait 对象，解耦实现
        ctx.service_registry.lock().unwrap()
            .register::<Box<dyn SlowLogService>>(Box::new(SlowLogServiceImpl::new(64, vec![])));
        // 注册命令处理器
        if let Some(reg) = ctx.command_registry.as_mut() {
            reg.register("slowlog-scan", Box::new(SlowLogCommandHandler {
                registry: Arc::clone(&ctx.service_registry),
            }));
            reg.register("slowlog-import", Box::new(SlowLogParseAndImportHandler {
                registry: Arc::clone(&ctx.service_registry),
            }));
        }
    }
}
