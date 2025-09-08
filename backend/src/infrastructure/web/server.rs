// Web Server Infrastructure

use axum::Router;
use tokio::signal;
use std::net::SocketAddr;
use tracing::info;
use microkernel::platform::message_bus::MessageBus;

use crate::interface::http::middleware;
use crate::interface::mcp::handlers::mcp_handler;

/// 启动带有优雅关闭的服务器
pub async fn start_server_with_shutdown(
    host: String,
    port: u16,
) -> anyhow::Result<()> {
    // 简化的MCP初始化
    init_mcp_service().await;
    
    // 创建完整的应用路由，包括 API 和静态文件
    let app = Router::new()
        // MCP 路由 - 使用统一的 tihc-mcp topic
        .route("/mcp", axum::routing::any(mcp_handler))
        // 集成新的 DDD API 路由
        .merge(crate::interface::http::routes::create_api_routes())
        // 静态文件路由（作为fallback处理SPA路由）
        .fallback(crate::interface::http::static_files::static_handler)
        .layer(middleware::cors_layer())
        .layer(middleware::trace_layer());

    let addr = SocketAddr::new(host.parse()?, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(host, port, "[TiHC] Web server started with unified MCP (topic: tihc-mcp)");

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async move {
            signal::ctrl_c().await.ok();
            info!("[TiHC] Web server received shutdown signal, exiting...");
        })
        .await?;
    Ok(())
}

/// 简化的MCP初始化
async fn init_mcp_service() {
    use microkernel::platform::message_bus::{BusMessage, GLOBAL_MESSAGE_BUS};
    
    // 发送初始化信号
    let msg = BusMessage::ok("tihc-mcp", serde_json::json!({
        "method": "register",
        "params": {}
    }));
    
    let _ = GLOBAL_MESSAGE_BUS.request(msg).await;
    tracing::info!("MCP service initialization completed");
}

/// 注册MCP处理器到消息总线
async fn register_mcp_handlers() -> anyhow::Result<()> {
    use microkernel::platform::message_bus::{BusMessage, GLOBAL_MESSAGE_BUS};
    
    // 发送初始化消息到统一的 tihc-mcp topic
    let msg = BusMessage::ok("tihc-mcp", serde_json::json!({
        "method": "register",
        "params": "server_starting"
    }));
    
    match GLOBAL_MESSAGE_BUS.request(msg).await {
        Ok(response) => {
            if response.is_ok() {
                tracing::info!("MCP unified handlers registered successfully");
                Ok(())
            } else {
                let error = response.data.error.unwrap_or_else(|| "Unknown error".to_string());
                tracing::warn!("MCP handler registration returned error: {}", error);
                Ok(()) // 不阻止服务器启动
            }
        }
        Err(e) => {
            tracing::warn!("Failed to communicate with MCP plugin via tihc-mcp topic: {}", e);
            Ok(()) // 不阻止服务器启动，MCP功能将按需注册
        }
    }
}
