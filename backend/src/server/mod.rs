use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

pub async fn start_server(host: String, port: u16) -> anyhow::Result<()> {
    // 构建 axum 路由，使用 api::create_router，确保所有 API 路由和中间件生效
    let app = crate::api::create_router(Arc::new(()));

    let addr = SocketAddr::new(host.parse()?, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(host, port, "[TiHC] Web server started");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;
    signal::ctrl_c().await.expect("failed to listen for ctrl_c");
}
