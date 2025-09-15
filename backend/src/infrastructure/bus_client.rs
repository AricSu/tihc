use microkernel::platform::message_bus::{BusMessage, Topic, GLOBAL_MESSAGE_BUS};
use serde_json::Value;
use std::time::Duration;
use anyhow::Result;

/// MCP/HTTP 与 Plugin 通用的 BusClient 抽象
#[derive(Clone)]
pub struct InfraBusClient;

impl InfraBusClient {
    pub fn new() -> Self {
        Self
    }

    /// 发送 Request（等待返回）
    pub async fn request<T: Into<Value>>(
        &self,
        topic: &str,
        sub: Option<&str>,
        data: T,
        timeout: Option<Duration>,
    ) -> Result<Value> {
        let topic = match sub {
            Some(sub) => Topic::new(topic, Some(sub)),
            None => Topic::new(topic, None::<&str>),
        };
        let bus = GLOBAL_MESSAGE_BUS.clone();
        let reply = bus.request(BusMessage::ok(topic, data), timeout).await?;
        Ok(reply.data)
    }

    /// 发送 Broadcast（无返回）
    pub async fn broadcast<T: Into<Value>>(
        &self,
        topic: &str,
        data: T,
    ) -> Result<()> {
        let topic = Topic::new(topic, None::<&str>);
        let bus = GLOBAL_MESSAGE_BUS.clone();
        bus.send(BusMessage::ok(topic, data)).await
    }
}
