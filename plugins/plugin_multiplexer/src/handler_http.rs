use std::sync::Arc;
use tokio::net::TcpStream;
use async_trait::async_trait;
use anyhow::Result;
use axum::Router;
use hyper::server::conn::http1;
use hyper_util::service::TowerToHyperService;
use hyper_util::rt::TokioIo;
use crate::ProtocolHandler;
use tracing;

pub struct HttpHandler {
    router: Arc<Router>,
}

impl HttpHandler {
    pub fn new(router: Arc<Router>) -> Self {
        Self { router }
    }
}

#[async_trait]
impl ProtocolHandler for HttpHandler {
    async fn handle(&self, stream: TcpStream) -> Result<()> {
        tracing::debug!(target: "plugin_multiplexer::HttpHandler", "HttpHandler invoked for new connection");
        let io = TokioIo::new(stream);
        let router = (*self.router).clone();
        let svc = TowerToHyperService::new(router);
        http1::Builder::new()
            .serve_connection(io, svc)
            .await?;
        Ok(())
    }
}
