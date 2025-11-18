use serde::{Serialize, Deserialize};

/// 插件相关事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    RegisterHttpRoute(RegisterHttpRoute),
    // ...可扩展更多事件
}

/// 注册 HTTP 路由事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterHttpRoute {
    pub path: String,
    // handler 不能序列化，这里只传 path，handler 由插件注册时通过全局注册表传递
}
use axum::{Router, Extension, routing::any, extract::Request, response::Response};

/// 构建插件分发路由，供主程序挂载
pub fn plugin_router(registry: std::sync::Arc<PluginRegistry>) -> Router<()> {
    Router::new()
        .route("/api/plugin/{*path}", any(plugin_dispatch_handler))
        .route("/", any(plugin_dispatch_handler))
        .route("/{*path}", any(plugin_dispatch_handler))
        .layer(Extension(registry))
}

/// 插件分发 handler
pub async fn plugin_dispatch_handler(
    Extension(reg): Extension<std::sync::Arc<PluginRegistry>>,
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
use std::sync::Arc;
use dashmap::DashMap;
use std::future::Future;
use std::pin::Pin;

pub type PluginHandler = Arc<dyn Fn(Request<axum::body::Body>) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>;

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
        self.routes.insert(path.to_string(), handler);
    }
    pub fn unregister_plugin(&self, plugin_prefix: &str) {
        let keys: Vec<_> = self.routes.iter().filter(|r| r.key().starts_with(plugin_prefix)).map(|r| r.key().clone()).collect();
        for k in keys {
            self.routes.remove(&k);
        }
    }
    pub fn get_handler(&self, path: &str) -> Option<PluginHandler> {
        self.routes.get(path).map(|h| h.value().clone())
    }
}
