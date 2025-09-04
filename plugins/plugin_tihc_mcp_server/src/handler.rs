use super::Counter;
use axum::Router;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use tracing;

pub async fn handle_streamable_http(stream: tokio::net::TcpStream) -> anyhow::Result<()> {
    let peer_addr = stream.peer_addr().ok();
    // 读取部分数据，尝试解析 session_id
    let mut buf = [0u8; 1024];
    let n = stream.peek(&mut buf).await.unwrap_or(0);
    let data = &buf[..n];
    let data_str = String::from_utf8_lossy(data);
    let mut session_id = None;
    for line in data_str.lines() {
        if line.to_ascii_lowercase().starts_with("mcp-session-id:") {
            session_id = Some(line[15..].trim());
            break;
        }
    }
    tracing::info!(target: "plugin_tihc_mcp_server", "[收到连接] mcp-session-id: {:?}", session_id);
    tracing::info!(target: "plugin_tihc_mcp_server", "Handling MCP streamable-http connection from {:?}", peer_addr);

    let service = StreamableHttpService::new(
        || Ok(Counter::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    // 注册根路径 / 和 /mcp，兼容所有 MCP streamable-http/SSE 客户端
    let router = Router::new()
        .fallback_service(service.clone())
        .nest_service("/mcp", service);
    let svc = TowerToHyperService::new(router.into_service());
    let io = TokioIo::new(stream);

    let result = http1::Builder::new().serve_connection(io, svc).await;

    match result {
        Ok(_) => {
            tracing::info!(target: "plugin_tihc_mcp_server", "MCP streamable-http connection closed for {:?}", peer_addr);
            Ok(())
        }
        Err(e) => {
            tracing::error!(target: "plugin_tihc_mcp_server", "MCP streamable-http handler error: {:?}, peer_addr={:?}", e, peer_addr);
            dbg!(&e);
            Err(anyhow::anyhow!(e))
        }
    }
}
