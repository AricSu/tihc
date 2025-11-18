use microkernel::plugin::{PluginRegistry, PluginHandler};
use axum::{extract::Request};
use axum::response::IntoResponse;
use crate::static_dist_router;
use tower::ServiceExt; // for .oneshot()
/// 注册静态资源路由到 microkernel 插件注册表
pub fn register_static_embed_route(registry: &PluginRegistry) {
    // 这里以 / 为例，实际可用 /static 前缀
    let base_router = static_dist_router();
    // 只注册 index.html 和 /*path
    for route in ["/", "/*path"] {
        let router = base_router.clone();
        let handler: PluginHandler = std::sync::Arc::new(
            move |req: Request<axum::body::Body>| {
                let router = router.clone();
                Box::pin(async move { router.oneshot(req).await.into_response() })
            },
        );
        registry.register_route(route, handler.clone());
    }
}

use microkernel::{EventBus};
use std::sync::Arc;
use tokio::task;

/// 注册业务 handler，订阅总线并处理事件
pub fn register_backend_handler(bus: Arc<EventBus<String>>) {
    // 订阅广播事件
    let mut rx = bus.subscribe();
    task::spawn(async move {
        while let Ok(envelope) = rx.recv().await {
            println!("[backend] 收到事件: type={}, payload={:?}", envelope.event_type, envelope.payload);
            // 这里可以做业务处理
        }
    });
}
