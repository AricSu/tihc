use crate::EventBus;
use inventory;
use std::sync::Arc;
use serde::{Deserialize, Serialize};


pub trait KernelPlugin: Send + Sync {
    fn register(&self, bus: Arc<EventBus<PluginEvent>>, registry: Arc<PluginRegistry>);
}

pub struct PluginFactory(pub fn() -> Box<dyn KernelPlugin>);

inventory::collect!(PluginFactory);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    RegisterHttpRoute(RegisterHttpRoute),
    GracefulShutdown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterHttpRoute {
    pub path: String,
}

use axum::{extract::Request, response::Response};
use dashmap::DashMap;
use std::future::Future;
use std::pin::Pin;

pub type PluginHandler = Arc<
    dyn Fn(Request<axum::body::Body>) -> Pin<Box<dyn Future<Output = Response> + Send>>
        + Send
        + Sync,
>;

pub struct PluginRegistry {
    pub routes: DashMap<String, PluginHandler>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            routes: DashMap::new(),
        }
    }
    pub fn register_route(&self, path: &str, handler: PluginHandler) {
        tracing::debug!(target: "plugin_registry", "Registering route: {}", path);
        self.routes.insert(path.to_string(), handler);
        let keys: Vec<_> = self.routes.iter().map(|r| r.key().clone()).collect();
        tracing::debug!(target: "plugin_registry", "Current registered routes: {:?}", keys);
    }
    pub fn unregister_plugin(&self, plugin_prefix: &str) {
        let keys: Vec<_> = self
            .routes
            .iter()
            .filter(|r| r.key().starts_with(plugin_prefix))
            .map(|r| r.key().clone())
            .collect();
        for k in keys {
            self.routes.remove(&k);
        }
    }
    pub fn get_handler(&self, path: &str) -> Option<PluginHandler> {
        tracing::debug!(target: "plugin_registry", "get_handler lookup: {}", path);
        // 优先精确匹配
        if let Some(h) = self.routes.get(path) {
            tracing::debug!(target: "plugin_registry", "Matched exact route: {}", path);
            return Some(h.value().clone());
        }
        // fallback 到 '/{*path}'
        if let Some(h) = self.routes.get("/{*path}") {
            tracing::debug!(target: "plugin_registry", "Matched wildcard route: /{{*path}} for {}", path);
            return Some(h.value().clone());
        }
        // fallback 到 '/'
        if let Some(h) = self.routes.get("/") {
            tracing::debug!(target: "plugin_registry", "Matched fallback route: / for {}", path);
            return Some(h.value().clone());
        }
        tracing::warn!(target: "plugin_registry", "No handler found for: {}", path);
        None
    }
}

use axum::{Router, Extension, routing::any};

/// 构建插件分发路由，供主程序挂载
pub fn plugin_router(registry: Arc<PluginRegistry>) -> Router<()> {
    Router::new()
        .route("/api/plugin/{*path}", any(plugin_dispatch_handler))
        .route("/", any(plugin_dispatch_handler))
        .route("/{*path}", any(plugin_dispatch_handler))
        .layer(Extension(registry))
}

/// 插件分发 handler
pub async fn plugin_dispatch_handler(
    Extension(reg): Extension<Arc<PluginRegistry>>,
    req: Request<axum::body::Body>,
) -> Response {
    let path = req.uri().path();
    tracing::info!(target: "plugin_dispatch", "Received HTTP request: {}", path);
    if let Some(handler) = reg.get_handler(path) {
        tracing::info!(target: "plugin_dispatch", "Dispatching to plugin handler for path: {}", path);
        handler(req).await
    } else {
        tracing::warn!(target: "plugin_dispatch", "No plugin handler found for path: {}", path);
        Response::builder().status(404).body(axum::body::Body::from("Not Found")).unwrap()
    }
}
