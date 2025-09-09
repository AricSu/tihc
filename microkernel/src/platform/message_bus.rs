use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use futures::executor::block_on;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// 支持闭包注册的 handler 类型（Box 包裹闭包，避免泛型污染 impl）
pub struct FnHandler {
    pub f: Box<dyn Fn(BusMessage) -> Result<BusMessage> + Send + Sync + 'static>,
}

#[async_trait]
impl MessageHandler for FnHandler {
    async fn handle(&self, msg: BusMessage) -> Result<BusMessage> {
        (self.f)(msg)
    }
}

/// 全局消息总线单例
pub static GLOBAL_MESSAGE_BUS: Lazy<Arc<MessageBusImpl>> =
    Lazy::new(|| Arc::new(MessageBusImpl::new()));

#[derive(Clone, Debug, serde::Serialize)]
pub struct MessageData {
    pub ok: bool,
    pub data: Option<Value>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct BusMessage {
    pub topic: String,
    pub data: MessageData,
}

impl BusMessage {
    /// 判断消息是否 ok
    pub fn is_ok(&self) -> bool {
        self.data.ok
    }

    /// 判断消息是否 error
    pub fn is_err(&self) -> bool {
        !self.data.ok
    }
    /// 构造 ok 消息
    pub fn ok<T: serde::Serialize>(topic: impl ToString, data: T) -> Self {
        Self {
            topic: topic.to_string(),
            data: MessageData {
                ok: true,
                data: Some(serde_json::to_value(data).unwrap_or(Value::Null)),
                error: None,
            },
        }
    }

    /// 链式/函数式设置 topic，可接受闭包
    pub fn with_topic<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }

    /// 构造 error 消息
    pub fn error(topic: impl ToString, err: impl ToString) -> Self {
        Self {
            topic: topic.to_string(),
            data: MessageData {
                ok: false,
                data: None,
                error: Some(err.to_string()),
            },
        }
    }

    /// 链式/函数式设置 data，可接受闭包
    pub fn with_data<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }

    /// 链式/函数式设置 error，可接受闭包
    pub fn with_error<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }

    /// pipe: 传递自身给闭包，返回闭包结果
    pub fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }

    /// when: 条件成立时执行闭包，链式/函数式
    pub fn when<F>(self, cond: bool, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if cond {
            f(self)
        } else {
            self
        }
    }

    /// then: ok 时执行闭包，链式/函数式
    pub fn then<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if self.data.ok {
            f(self)
        } else {
            self
        }
    }

    /// unwrap_data: 取出 data 字段，返回 Result
    pub fn unwrap_data<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        self.data
            .data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("BusMessage data missing"))
            .and_then(|v| serde_json::from_value(v.clone()).map_err(|e| anyhow::anyhow!(e)))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HandlerMode {
    Broadcast,
    Request,
}

/// 消息处理器 trait
#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle(&self, msg: BusMessage) -> Result<BusMessage>;
}

/// 消息总线 trait
#[async_trait]
pub trait MessageBus: Send + Sync {
    async fn register(&self, topic: &str, handler: Arc<dyn MessageHandler>, mode: HandlerMode);
    async fn send(&self, msg: BusMessage) -> Result<Vec<BusMessage>>;
    async fn request(&self, msg: BusMessage) -> Result<BusMessage>;

    /// 支持超时的 request 方法（MCP 统一 API 层使用）
    async fn request_with_timeout(
        &self,
        topic: &str,
        data: Value,
        timeout_duration: Duration,
    ) -> Result<BusMessage>;
}

/// 消息总线实现
pub struct MessageBusImpl {
    subscribers: DashMap<String, Vec<Arc<dyn MessageHandler>>>,
    request_handlers: DashMap<String, Arc<dyn MessageHandler>>,
}

impl MessageBusImpl {
    /// 构造
    pub fn new() -> Self {
        Self {
            subscribers: DashMap::new(),
            request_handlers: DashMap::new(),
        }
    }

    // --- 注册 handler 语法糖 ---
    /// 链式注册 handler（mode 默认 Broadcast）
    pub fn register_chain(
        self: Arc<Self>,
        topic: &str,
        handler: Arc<dyn MessageHandler>,
        mode: Option<HandlerMode>,
    ) -> Arc<Self> {
        let m = mode.unwrap_or(HandlerMode::Broadcast);
        block_on(self.register(topic, handler, m));
        self
    }

    /// 函数式注册 handler（闭包，mode 默认 Broadcast）
    pub fn register_fn<F>(
        self: Arc<Self>,
        topic: &str,
        f: F,
        mode: Option<HandlerMode>,
    ) -> Arc<Self>
    where
        F: Fn(BusMessage) -> Result<BusMessage> + Send + Sync + 'static,
    {
        let m = mode.unwrap_or(HandlerMode::Broadcast);
        let handler = Arc::new(FnHandler { f: Box::new(f) });
        block_on(self.register(topic, handler, m));
        self
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

    async fn request_with_timeout(
        &self,
        topic: &str,
        data: Value,
        timeout_duration: Duration,
    ) -> Result<BusMessage> {
        let msg = BusMessage::ok(topic, data);
        let request_future = async {
            if let Some(handler) = self.request_handlers.get(topic) {
                handler.handle(msg).await
            } else {
                Err(anyhow::anyhow!("No handler for topic {}", topic))
            }
        };
        timeout(timeout_duration, request_future)
            .await
            .map_err(|_| anyhow::anyhow!("Request timeout after {:?}", timeout_duration))?
    }
}
