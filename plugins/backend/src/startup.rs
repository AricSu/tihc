use axum::response::IntoResponse;
use microkernel::plugin::{PluginEvent, RegisterHttpRoute, PluginHandler};
use microkernel::{EventBus, EventEnvelope};
use std::sync::Arc;
use crate::static_dist_router;
use axum::extract::Request;
use tower::ServiceExt;

/// 插件启动时通过 EventBus 注册静态 embed 路由
pub fn register_static_embed_via_bus(bus: Arc<EventBus<PluginEvent>>, registry: Arc<microkernel::plugin::PluginRegistry>) {
    tracing::info!(target: "backend", "register_static_embed_via_bus called");
    let static_router = static_dist_router();
    let handler: PluginHandler = Arc::new(
        move |req: Request<axum::body::Body>| {
            let router = static_router.clone();
            Box::pin(async move { router.oneshot(req).await.into_response() })
        },
    );
    // 直接同步注册 handler，避免竞态
    for path in ["/", "/{*path}"] {
        tracing::info!(target: "backend", "Registering static embed route: {}", path);
        registry.register_route(path, handler.clone());
        // 事件仅用于通知/日志
        let reg_event = EventEnvelope::new(
            "plugin_register_http_route",
            PluginEvent::RegisterHttpRoute(RegisterHttpRoute {
                path: path.to_string(),
            }),
            None,
        );
        let _ = bus.broadcast(reg_event);
    }
}
