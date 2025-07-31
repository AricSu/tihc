/// 启动 web 服务，支持外部传入 shutdown_rx，实现统一优雅关闭
pub async fn start_server_with_shutdown(
host: String,
port: u16,
command_registry: CommandRegistry,
mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> anyhow::Result<()> {
    let mut service_registry = ServiceRegistry::new();
    service_registry.register(Box::new(command_registry));
    let app = crate::api::create_router(Arc::new(service_registry));

    let addr = SocketAddr::new(host.parse()?, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(host, port, "[TiHC] Web server started (with external shutdown)");

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
            info!("[TiHC] Web server received shutdown signal, exiting...");
        })
        .await?;
    Ok(())
}
use core::platform::ServiceRegistry;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

use core::platform::command_registry::CommandRegistry;

pub async fn start_server(
    host: String,
    port: u16,
    command_registry: CommandRegistry,
) -> anyhow::Result<()> {
    // 构建 axum 路由，使用 api::create_router，确保所有 API 路由和中间件生效
    let mut service_registry = ServiceRegistry::new();
    // 注册 CommandRegistry 到 ServiceRegistry，供 API handler resolve
    service_registry.register(Box::new(command_registry));
    let app = crate::api::create_router(Arc::new(service_registry));

    let addr = SocketAddr::new(host.parse()?, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(host, port, "[TiHC] Web server started");
    // Create a broadcast channel for shutdown signal
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    // Spawn a task to listen for Ctrl+C and send shutdown signal
    tokio::spawn(async move {
        shutdown_signal(&shutdown_tx).await;
    });

    // Pass shutdown_rx to axum for graceful shutdown
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
        })
        .await?;
    Ok(())
}

/// 等待 Ctrl+C 并广播 shutdown 信号
pub async fn shutdown_signal(shutdown_service: &tokio::sync::broadcast::Sender<()>) {
    tokio::signal::ctrl_c().await.expect("failed to listen for ctrl_c");
    let _ = shutdown_service.send(());
}
