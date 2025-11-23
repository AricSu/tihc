use crate::infrastructure::startup::create_infra_state;
use microkernel::plugin::{PluginEvent, PluginHandler, PluginRegistry, RegisterHttpRoute};
use microkernel::{EventBus, EventEnvelope};
use std::sync::Arc;
use tower::ServiceExt;
use axum::extract::Request;
use axum::response::IntoResponse;
use crate::interface::http::{create_api_routes, static_embed};

/// register static embedded files via EventBus and PluginRegistry
pub fn register_static_embed_via_bus(
    bus: Arc<EventBus<PluginEvent>>,
    registry: Arc<PluginRegistry>,
) {
    tracing::info!(target: "backend", "register_static_embed_via_bus called");
    let static_router = static_embed::static_dist_router();
    let handler: PluginHandler = Arc::new(move |req: Request<axum::body::Body>| {
        let router = static_router.clone();
        Box::pin(async move { router.oneshot(req).await.into_response() })
    });
    // register routes
    for path in ["/", "/{*path}"] {
        tracing::info!(target: "backend", "Registering static embed route: {}", path);
        registry.register_route(path, handler.clone());
        // use EventBus to broadcast registration event
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

/// 注册 API 路由到微内核 PluginRegistry，并广播事件
pub async fn register_api_routes_via_bus(
    bus: Arc<EventBus<PluginEvent>>,
    registry: Arc<PluginRegistry>,
    config_value: &toml::Value,
) {
    // 1. 创建 AppState
    let app_state = Arc::new(create_infra_state(config_value).await.expect("Failed to create infra state"));
    // 2. 构建 API Router
    let api_router = create_api_routes(app_state);

    // 最佳实践：只注册 /api/{*path}，由 axum router 内部分发所有 API 路由
    // 微内核只做入口分发，业务路由由 axum 层管理
    let handler: PluginHandler = Arc::new(move |req: Request<axum::body::Body>| {
        let router = api_router.clone();
        Box::pin(async move { router.oneshot(req).await.into_response() })
    });
    let path = "/api/{*path}";
    registry.register_route(path, handler.clone());
    let reg_event = EventEnvelope::new(
        "plugin_register_http_route",
        PluginEvent::RegisterHttpRoute(RegisterHttpRoute {
            path: path.to_string(),
        }),
        None,
    );
    let _ = bus.broadcast(reg_event);
}