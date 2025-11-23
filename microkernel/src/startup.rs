use axum::{Router, extract::Request, response::IntoResponse};
use crate::plugin::{PluginEvent, PluginRegistry};
use crate::EventEnvelope;
use crate::EventBus;
use std::net::SocketAddr;
use std::sync::Arc;

/// 启动 axum 服务，事件驱动动态路由分发，主服务与插件完全解耦
pub async fn run_axum_server(
	listen: String,
	port: u16,
	registry: Arc<PluginRegistry>,
	bus: Option<Arc<EventBus<PluginEvent>>>,
) -> anyhow::Result<()> {
	// 1. 监听 EventBus，动态注册路由到 PluginRegistry
	if let Some(bus) = bus.clone() {
		let mut bus_rx = bus.subscribe();
		tokio::spawn(async move {
			while let Ok(event) = bus_rx.recv().await {
				if let PluginEvent::RegisterHttpRoute(reg) = event.payload {
					// 这里只注册路径，handler需插件自行注册
					tracing::info!(target: "microkernel", "[microkernel] Registered plugin HTTP route: {} (event)", reg.path);
					// registry_clone.register_route(&reg.path, handler);
				}
			}
		});
	}

	// 2. 构建 axum Router，仅挂一个 handler，所有路由走 PluginRegistry，支持所有 HTTP 方法
	let app = Router::new().fallback(
		axum::routing::any(|req: Request<axum::body::Body>| async move {
			let path = req.uri().path().to_string();
			if let Some(handler) = registry.get_handler(&path) {
				handler(req).await.into_response()
			} else {
				axum::http::StatusCode::NOT_FOUND.into_response()
			}
		})
	);

	let addr: SocketAddr = format!("{}:{}", listen, port).parse()?;
	tracing::info!("tihc microkernel server listening on {}", addr);
	let listener = tokio::net::TcpListener::bind(addr).await?;
	axum::serve(listener, app.into_make_service())
		.with_graceful_shutdown(async move {
			tokio::signal::ctrl_c().await.ok();
			tracing::info!("[TiHC] Web server received shutdown signal, exiting...");
			if let Some(bus) = bus {
				let shutdown_event = EventEnvelope::new(
					"plugin_shutdown",
					PluginEvent::GracefulShutdown,
					None,
				);
				let _ = bus.broadcast(shutdown_event);
			}
		})
		.await?;
	Ok(())
}
