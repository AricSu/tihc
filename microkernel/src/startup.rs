use axum::Router;
use std::net::SocketAddr;

/// build axum app with optional plugin router
pub fn build_app(plugin_router: Option<Router<()>>) -> Router<()> {
	let mut app = Router::new();
	if let Some(router) = plugin_router {
		app = app.merge(router);
	}
	app
}

pub async fn run_axum_server(listen: String, port: u16, plugin_router: Option<Router<()>>) -> anyhow::Result<()> {
	let app = build_app(plugin_router);
	let addr: SocketAddr = format!("{}:{}", listen, port).parse()?;
	tracing::info!("tihc server listening on {}", addr);
	let listener = tokio::net::TcpListener::bind(addr).await?;
	axum::serve(listener, app.into_make_service())
		.with_graceful_shutdown(async move {
			tokio::signal::ctrl_c().await.ok();
			tracing::info!("[TiHC] Web server received shutdown signal, exiting...");
		})
		.await?;
	Ok(())
}
