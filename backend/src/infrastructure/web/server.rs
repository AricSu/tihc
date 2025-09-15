use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

use crate::interface::{http::middleware, TihcMcpServer};

pub async fn start_server_with_shutdown(host: String, port: u16) -> anyhow::Result<()> {
    // 创建 rmcp streamable 服务
    let rmcp_service = rmcp::transport::streamable_http_server::StreamableHttpService::new(
        || Ok(TihcMcpServer::new()),
        rmcp::transport::streamable_http_server::session::local::LocalSessionManager::default()
            .into(),
        Default::default(),
    );

    // 创建 axum 路由，/mcp 走 rmcp streamable，其余走原有 API/静态路由
    let app = axum::Router::new()
        .nest_service("/mcp", rmcp_service)
        .merge(crate::interface::http::routes::create_api_routes())
        .fallback(crate::interface::http::static_files::static_handler)
        .layer(middleware::cors_layer())
        .layer(middleware::trace_layer());

    let addr = SocketAddr::new(host.parse()?, port);
    tracing::info!(
        "Web server started with unified MCP (topic: tihc-mcp) at {}",
        addr
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async move {
            signal::ctrl_c().await.ok();
            info!("[TiHC] Web server received shutdown signal, exiting...");
        })
        .await?;
    Ok(())
}
