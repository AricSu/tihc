pub mod plugin;

pub mod handler_http;
pub use handler_http::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use async_trait::async_trait;
use anyhow::Result;

// 协议类型标识
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ProtocolKind {
    Http,                  // 普通 HTTP/RESTful/SSE/streamable-http-server
    RmcpStreamableHttp,    // rmcp 的 streamable-http
    RmcpSse,               // rmcp 的 SSE
    CustomBinary,          // 其他自定义二进制协议
    // ...
}

#[async_trait]
pub trait ProtocolHandler: Send + Sync + 'static {
    async fn handle(&self, stream: TcpStream) -> Result<()>;
}

#[derive(Clone)]
pub struct Multiplexer {
    handlers: HashMap<ProtocolKind, Arc<dyn ProtocolHandler>>,
}

impl Default for Multiplexer {
    fn default() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
}

impl Multiplexer {

    pub fn register_handler(&mut self, kind: ProtocolKind, handler: Arc<dyn ProtocolHandler>) {
        self.handlers.insert(kind, handler);
    }

    pub async fn run(&self, listen_addr: &str) -> Result<()> {
        let listener = TcpListener::bind(listen_addr).await?;
        tracing::info!(target: "plugin_multiplexer", "Multiplexer listening on {}", listen_addr);
        loop {
            let (mut stream, addr) = listener.accept().await?;
            tracing::debug!(target: "plugin_multiplexer", "Accepted new connection from {}", addr);
            let handlers = self.handlers.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 16];
                let n = match stream.peek(&mut buf).await {
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("peek error: {e}");
                        return;
                    }
                };
                let proto = detect_protocol(&buf[..n]);
                tracing::debug!(target: "plugin_multiplexer", "Protocol detected: {:?} (first bytes: {:02X?})", proto, &buf[..n]);
                match proto {
                    ProtocolKind::Http => {
                        if let Some(handler) = handlers.get(&ProtocolKind::Http) {
                            let _ = handler.handle(stream).await;
                        }
                    }
                    ProtocolKind::RmcpStreamableHttp => {
                        if let Some(handler) = handlers.get(&ProtocolKind::RmcpStreamableHttp) {
                            let _ = handler.handle(stream).await;
                        }
                    }
                    ProtocolKind::RmcpSse => {
                        if let Some(handler) = handlers.get(&ProtocolKind::RmcpSse) {
                            let _ = handler.handle(stream).await;
                        }
                    }
                    ProtocolKind::CustomBinary => {
                        if let Some(handler) = handlers.get(&ProtocolKind::CustomBinary) {
                            let _ = handler.handle(stream).await;
                        }
                    }
                }
            });
        }
    }
}

// 协议判别器示例
pub fn detect_protocol(buf: &[u8]) -> ProtocolKind {
    // 简单示例：根据首字节判别
    if buf.starts_with(b"GET ") || buf.starts_with(b"POST ") || buf.starts_with(b"HTTP/") {
        ProtocolKind::Http
    } else if buf.starts_with(b"RMCP-STREAM") {
        ProtocolKind::RmcpStreamableHttp
    } else if buf.starts_with(b"RMCP-SSE") {
        ProtocolKind::RmcpSse
    } else {
        ProtocolKind::CustomBinary
    }
}
