use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use futures::future::join_all;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, warn, error};

/// -------------------- Topic --------------------
#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Topic {
    pub main: String,
    pub sub: String, // 默认 "default"
}

impl Topic {
    pub fn new(main: impl ToString, sub: Option<impl ToString>) -> Self {
        Self {
            main: main.to_string(),
            sub: sub.map(|s| s.to_string()).unwrap_or_else(|| "default".into()),
        }
    }
}

#[macro_export]
macro_rules! topic {
    ($main:expr) => {
        $crate::platform::message_bus::Topic::new($main, None::<String>)
    };
    ($main:expr, $sub:expr) => {
        $crate::platform::message_bus::Topic::new($main, $sub)
    };
}

/// -------------------- BusMessage --------------------
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BusMessage {
    pub topic: Topic,
    pub data: Value,
}

impl BusMessage {
    pub fn ok(topic: Topic, data: impl Into<Value>) -> Self {
        Self { topic, data: data.into() }
    }
}

/// -------------------- HandlerMode --------------------
#[derive(Clone, Copy, Debug)]
pub enum HandlerMode {
    Broadcast,
    Request,
}

/// -------------------- MessageHandler --------------------
#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle(&self, msg: BusMessage) -> Result<BusMessage>;
}

/// -------------------- FnHandler --------------------
pub struct FnHandler {
    f: Box<dyn Fn(BusMessage) -> Result<BusMessage> + Send + Sync>,
}

#[async_trait]
impl MessageHandler for FnHandler {
    async fn handle(&self, msg: BusMessage) -> Result<BusMessage> {
        (self.f)(msg)
    }
}

/// -------------------- MessageBusImpl --------------------
pub struct MessageBusImpl {
    broadcast: DashMap<String, Vec<Arc<dyn MessageHandler>>>,
    request: DashMap<(String, String), Arc<dyn MessageHandler>>, // (main, sub)
    default_timeout: Duration,
}

impl MessageBusImpl {
    pub fn new() -> Self {
        Self {
            broadcast: DashMap::new(),
            request: DashMap::new(),
            default_timeout: Duration::from_secs(5),
        }
    }

    pub fn with_timeout(mut self, dur: Duration) -> Self {
        self.default_timeout = dur;
        self
    }

    /// 注册 handler
    pub async fn register(&self, topic: &Topic, handler: Arc<dyn MessageHandler>, mode: HandlerMode) {
        match mode {
            HandlerMode::Broadcast => {
                self.broadcast.entry(topic.main.clone()).or_default().push(handler.clone());
                debug!("Registered Broadcast handler: {} / {}", topic.main, topic.sub);
            }
            HandlerMode::Request => {
                self.request.insert((topic.main.clone(), topic.sub.clone()), handler.clone());
                debug!("Registered Request handler: {} / {}", topic.main, topic.sub);
            }
        }
    }

    /// Broadcast 模式（并发执行，忽略返回值，但记录错误）
    pub async fn send(&self, msg: BusMessage) -> Result<()> {
        if let Some(subs) = self.broadcast.get(&msg.topic.main) {
            let futures: Vec<_> = subs.value().iter().map(|h| {
                let msg = msg.clone();
                async move {
                    if let Err(e) = h.handle(msg.clone()).await {
                        error!("Broadcast handler error on topic {}: {:?}", msg.topic.main, e);
                    } else {
                        debug!("Broadcast handler success for topic {}", msg.topic.main);
                    }
                }
            }).collect();
            join_all(futures).await;
        } else {
            debug!("No Broadcast subscribers for topic={}", msg.topic.main);
        }
        Ok(())
    }

    /// Request 模式，支持可选超时
    pub async fn request(&self, msg: BusMessage, dur: Option<Duration>) -> Result<BusMessage> {
        let fut = async {
            if let Some(h) = self.request.get(&(msg.topic.main.clone(), msg.topic.sub.clone())) {
                match h.handle(msg.clone()).await {
                    Ok(resp) => {
                        debug!("Request handler success for topic {} / {}", msg.topic.main, msg.topic.sub);
                        Ok(resp)
                    }
                    Err(e) => {
                        error!("Request handler error on topic {} / {}: {:?}", msg.topic.main, msg.topic.sub, e);
                        Err(e)
                    }
                }
            } else if let Some(h) = self.request.get(&(msg.topic.main.clone(), "default".into())) {
                match h.handle(msg.clone()).await {
                    Ok(resp) => {
                        debug!("Request handler success for topic {} / default", msg.topic.main);
                        Ok(resp)
                    }
                    Err(e) => {
                        error!("Request handler error on topic {} / default: {:?}", msg.topic.main, e);
                        Err(e)
                    }
                }
            } else {
                warn!("No Request handler for topic={} sub_topic={}", msg.topic.main, msg.topic.sub);
                Err(anyhow::anyhow!("No handler for topic {:?}", msg.topic))
            }
        };
        let timeout_duration = dur.unwrap_or(self.default_timeout);
        match timeout(timeout_duration, fut).await {
            Ok(res) => res,
            Err(_) => {
                error!("Request timeout for topic {} / {} after {:?}", msg.topic.main, msg.topic.sub, timeout_duration);
                Err(anyhow::anyhow!("Request timeout for topic {:?}", msg.topic))
            }
        }
    }
}

/// -------------------- Fluent API --------------------
#[derive(Clone)]
pub struct BusClient {
    bus: Arc<MessageBusImpl>,
}

impl BusClient {
    pub fn new() -> Self { Self { bus: GLOBAL_MESSAGE_BUS.clone() } }

    pub fn register_broadcast<F>(&self, topic: Topic, f: F) -> &Self
    where F: Fn(BusMessage) -> Result<BusMessage> + Send + Sync + 'static {
        let h = Arc::new(FnHandler { f: Box::new(f) });
        block_on(self.bus.register(&topic, h, HandlerMode::Broadcast));
        self
    }

    pub fn register_request<F>(&self, topic: Topic, f: F) -> &Self
    where F: Fn(BusMessage) -> Result<BusMessage> + Send + Sync + 'static {
        let h = Arc::new(FnHandler { f: Box::new(f) });
        block_on(self.bus.register(&topic, h, HandlerMode::Request));
        self
    }

    pub async fn send_broadcast(&self, topic: Topic, data: Value) -> Result<()> {
        self.bus.send(BusMessage::ok(topic, data)).await
    }

    pub async fn send_request(&self, topic: Topic, data: Value, timeout: Option<Duration>) -> Result<BusMessage> {
        self.bus.request(BusMessage::ok(topic, data), timeout).await
    }
}

/// -------------------- 全局单例 --------------------
pub static GLOBAL_MESSAGE_BUS: Lazy<Arc<MessageBusImpl>> = Lazy::new(|| Arc::new(MessageBusImpl::new()));

/// -------------------- block_on helper --------------------
fn block_on<F: std::future::Future>(f: F) -> F::Output {
    futures::executor::block_on(f)
}
