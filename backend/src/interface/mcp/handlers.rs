use axum::{
    extract::Request,
    response::Response,
    http::StatusCode,
    body::Body,
};
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, 
    session::local::LocalSessionManager,
};
use std::sync::Arc;
use tower::ServiceExt;
use http_body_util::BodyExt;
use crate::interface::mcp::service::SimpleMcpProxy;

pub async fn mcp_handler(req: Request<Body>) -> Result<Response<Body>, StatusCode> {
    // 使用静态的会话管理器来保持会话状态
    static SESSION_MANAGER: std::sync::OnceLock<Arc<LocalSessionManager>> = std::sync::OnceLock::new();
    let session_manager = SESSION_MANAGER.get_or_init(|| Arc::new(LocalSessionManager::default()));
    
    // 使用简化的MCP代理服务
    static SIMPLE_PROXY: std::sync::OnceLock<Arc<SimpleMcpProxy>> = std::sync::OnceLock::new();
    let proxy_service = SIMPLE_PROXY.get_or_init(|| Arc::new(SimpleMcpProxy::new()));
    
    let service = StreamableHttpService::new(
        {
            let proxy_service = proxy_service.clone();
            move || Ok((*proxy_service).clone())
        },
        session_manager.clone(),
        Default::default(),
    );
    
    // 使用 oneshot 处理单个请求
    match service.oneshot(req).await {
        Ok(response) => {
            // 转换 body 类型
            let (parts, body) = response.into_parts();
            let body_bytes = match body.collect().await {
                Ok(collected) => collected.to_bytes(),
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };
            Ok(Response::from_parts(parts, Body::from(body_bytes)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
