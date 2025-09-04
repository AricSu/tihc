use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::Arc;

pub static GLOBAL_MESSAGE_BUS: Lazy<Arc<MessageBusImpl>> =
    Lazy::new(|| Arc::new(MessageBusImpl::new()));

#[derive(Clone, Debug)]
pub struct BusMessage {
    pub topic: String,
    pub data: Value,
}

#[derive(Clone, Copy, Debug)]
pub enum HandlerMode {
    Broadcast,
    Request,
}

#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle(&self, msg: BusMessage) -> Result<BusMessage>;
}

#[async_trait]
pub trait MessageBus: Send + Sync {
    async fn register(&self, topic: &str, handler: Arc<dyn MessageHandler>, mode: HandlerMode);
    async fn send(&self, msg: BusMessage) -> Result<Vec<BusMessage>>;
    async fn request(&self, msg: BusMessage) -> Result<BusMessage>;
}

pub struct MessageBusImpl {
    subscribers: DashMap<String, Vec<Arc<dyn MessageHandler>>>,
    request_handlers: DashMap<String, Arc<dyn MessageHandler>>,
}

impl MessageBusImpl {
    pub fn new() -> Self {
        Self {
            subscribers: DashMap::new(),
            request_handlers: DashMap::new(),
        }
    }
}

#[async_trait]
impl MessageBus for MessageBusImpl {
    async fn register(&self, topic: &str, handler: Arc<dyn MessageHandler>, mode: HandlerMode) {
        match mode {
            HandlerMode::Broadcast => {
                self.subscribers
                    .entry(topic.to_string())
                    .or_insert_with(Vec::new)
                    .push(handler);
            }
            HandlerMode::Request => {
                self.request_handlers.insert(topic.to_string(), handler);
            }
        }
    }

    async fn send(&self, msg: BusMessage) -> Result<Vec<BusMessage>> {
        if let Some(subs) = self.subscribers.get(&msg.topic) {
            let mut results = Vec::new();
            for handler in subs.value().iter() {
                let reply = handler.handle(msg.clone()).await?;
                results.push(reply);
            }
            Ok(results)
        } else {
            Ok(vec![])
        }
    }

    async fn request(&self, msg: BusMessage) -> Result<BusMessage> {
        let topic = &msg.topic;
        if let Some(handler) = self.request_handlers.get(topic) {
            handler.handle(msg).await
        } else {
            Err(anyhow::anyhow!("No handler for topic {}", topic))
        }
    }
}
