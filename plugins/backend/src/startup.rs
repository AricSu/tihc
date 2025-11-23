use axum::response::IntoResponse;
use microkernel::plugin::{PluginEvent, RegisterHttpRoute, PluginHandler, PluginRegistry};
use microkernel::{EventBus, EventEnvelope};
use std::sync::Arc;
use axum::extract::Request;
use tower::ServiceExt;

use crate::interface::http::static_embed;

/// register static embedded files via EventBus and PluginRegistry
pub fn register_static_embed_via_bus(bus: Arc<EventBus<PluginEvent>>, registry: Arc<PluginRegistry>) {
    tracing::info!(target: "backend", "register_static_embed_via_bus called");
    let static_router = static_embed::static_dist_router();
    let handler: PluginHandler = Arc::new(
        move |req: Request<axum::body::Body>| {
            let router = static_router.clone();
            Box::pin(async move { router.oneshot(req).await.into_response() })
        },
    );
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
