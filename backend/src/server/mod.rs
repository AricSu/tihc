use axum::Router;
use microkernel::platform::ServiceRegistry;
use microkernel::platform::command_registry::CommandRegistry;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

pub async fn start_server_with_shutdown(
    host: String,
    port: u16,
    command_registry: CommandRegistry,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> anyhow::Result<()> {
    let mut service_registry = ServiceRegistry::new();
    service_registry.register(Box::new(command_registry));

    // 创建完整的应用路由，包括 API 和静态文件
    let app = Router::new()
        .nest(
            "/api",
            crate::api::create_router(Arc::new(service_registry)),
        )
        .merge(crate::api::static_routes::static_routes());

    let addr = SocketAddr::new(host.parse()?, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(
        host,
        port, "[TiHC] Web server started (with external shutdown)"
    );

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
            info!("[TiHC] Web server received shutdown signal, exiting...");
        })
        .await?;
    Ok(())
}
