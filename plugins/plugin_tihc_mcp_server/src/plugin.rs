use axum::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use crate::Counter;
use microkernel::plugin_api::traits::{Plugin, PluginContext};

impl Plugin for Counter {
    fn name(&self) -> &str { "plugin_tihc_mcp" }
    fn register(&mut self, ctx: &mut PluginContext) {
        // 兼容官方 streamhttp 示例
        let service = StreamableHttpService::new(
            || Ok(Counter::new()),
            LocalSessionManager::default().into(),
            Default::default(),
        );
        let mcp_router = Router::new().nest_service("/mcp", service);
        use std::sync::Arc;
        if let Ok(mut reg) = ctx.service_registry.lock() {
            // 尝试获取已注册的 Arc<Router>，有则 merge，无则新建
            if let Some(existing) = reg.resolve::<Arc<Router>>() {
                let merged = Arc::new(existing.as_ref().clone().merge(mcp_router));
                reg.register::<Arc<Router>>(merged);
            } else {
                reg.register::<Arc<Router>>(Arc::new(mcp_router));
            }
        }
    }
}