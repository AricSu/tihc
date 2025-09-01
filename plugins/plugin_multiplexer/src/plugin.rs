


use std::sync::Arc;
use microkernel::plugin_api::traits::{Plugin, PluginContext};
use crate::{Multiplexer, ProtocolKind};
use crate::handler_http::HttpHandler;

// use plugin_tihc_backend::api;

pub struct MultiplexerPlugin {
	multiplexer: Option<Arc<Multiplexer>>,
}

impl MultiplexerPlugin {
	pub fn new(service_registry: std::sync::Arc<std::sync::Mutex<microkernel::platform::ServiceRegistry>>) -> Self {
		Self::with_backend_router_and_registry(None, service_registry)
	}

	pub fn with_backend_router(router: Arc<axum::Router>, service_registry: std::sync::Arc<std::sync::Mutex<microkernel::platform::ServiceRegistry>>) -> Self {
		Self::with_backend_router_and_registry(Some(router), service_registry)
	}

	fn with_backend_router_and_registry(router: Option<Arc<axum::Router>>, service_registry: std::sync::Arc<std::sync::Mutex<microkernel::platform::ServiceRegistry>>) -> Self {
		use axum::Router;
		let mut merged_router = Router::new();
		let reg = service_registry.lock().unwrap();
		// 1. backend router
		let mut backend_router_opt = None;
		if let Some(r) = router {
			merged_router = merged_router.merge(r.as_ref().clone());
			backend_router_opt = Some(r.clone());
		} else if let Some(backend) = reg.resolve::<Arc<Router>>() {
			merged_router = merged_router.merge(backend.as_ref().clone());
			backend_router_opt = Some(backend.clone());
		}
		// 2. MCP router（假设 MCP 插件注册的 Router 也用 Arc<Router> 类型注册，且与 backend 不同实例）
		if let Some(mcp_router) = reg.resolve::<Arc<Router>>() {
			let need_merge = match &backend_router_opt {
				Some(backend) => !Arc::ptr_eq(&mcp_router, backend),
				None => true,
			};
			if need_merge {
				merged_router = merged_router.merge(mcp_router.as_ref().clone());
			}
		}
		let multiplexer = {
			let mut m = Multiplexer::default();
			let http_handler = Arc::new(HttpHandler::new(Arc::new(merged_router)));
			m.register_handler(ProtocolKind::Http, http_handler);
			m
		};
		Self {
			multiplexer: Some(Arc::new(multiplexer)),
		}
	}

	pub fn multiplexer(&self) -> Option<Arc<Multiplexer>> {
		self.multiplexer.clone()
	}
}

impl Plugin for MultiplexerPlugin {
	fn name(&self) -> &str {
		"plugin_multiplexer"
	}

	fn register(&mut self, ctx: &mut PluginContext) {
		if let Some(multiplexer) = &self.multiplexer {
			if let Ok(mut registry) = ctx.service_registry.lock() {
				registry.register::<Arc<Multiplexer>>(multiplexer.clone());
			}
		}
	}
}
