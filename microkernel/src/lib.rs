pub mod config;
pub mod log;
pub mod startup;
pub mod plugin;
pub use startup::run_axum_server;
pub use log::*;
pub use plugin::{PluginRegistry, PluginHandler};

use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{broadcast, mpsc, oneshot};
use uuid::Uuid;

/// 事件信封，封装事件元数据
#[derive(Debug, Clone)]
pub struct EventEnvelope<T> {
	pub trace_id: Uuid,
	pub session_id: Option<Uuid>,
	pub event_type: String,
	pub timestamp: SystemTime,
	pub payload: T,
}

impl<T> EventEnvelope<T> {
	pub fn new(event_type: impl Into<String>, payload: T, session_id: Option<Uuid>) -> Self {
		Self {
			trace_id: Uuid::new_v4(),
			session_id,
			event_type: event_type.into(),
			timestamp: SystemTime::now(),
			payload,
		}
	}
}

/// 消息总线（EventBus）
pub struct EventBus<T: Clone + Send + 'static> {
	broadcast_tx: broadcast::Sender<EventEnvelope<T>>,
	rpc_tx: mpsc::Sender<(EventEnvelope<T>, oneshot::Sender<T>)>,
}

impl<T: Clone + Send + 'static> EventBus<T> {
	/// 创建新的 EventBus，指定广播通道容量和 RPC 队列容量
	pub fn new(broadcast_capacity: usize, rpc_capacity: usize) -> Arc<Self> {
		let (broadcast_tx, _) = broadcast::channel(broadcast_capacity);
	let (rpc_tx, mut rpc_rx) = mpsc::channel::<(EventEnvelope<T>, oneshot::Sender<T>)>(rpc_capacity);
	let bus = Arc::new(Self { broadcast_tx, rpc_tx });
	let bus_clone = Arc::clone(&bus);
		// 启动 RPC 处理任务（需注册 handler）
		tokio::spawn(async move {
			// 这里只是占位，实际 handler 需通过 register_rpc_handler 注册
			while let Some((_envelope, _resp_tx)) = rpc_rx.recv().await {
				// 用户应实现 handler
			}
		});
		bus_clone
	}
	/// 广播事件，所有订阅者都能收到
	pub fn broadcast(&self, envelope: EventEnvelope<T>) -> Result<usize, broadcast::error::SendError<EventEnvelope<T>>> {
		self.broadcast_tx.send(envelope)
	}
	/// 订阅广播事件
	pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope<T>> {
		self.broadcast_tx.subscribe()
	}
	/// 发起 RPC 请求，等待响应
	pub async fn rpc(&self, envelope: EventEnvelope<T>) -> Result<T, String> {
		let (resp_tx, resp_rx) = oneshot::channel();
		self.rpc_tx.send((envelope, resp_tx)).await.map_err(|e| e.to_string())?;
		resp_rx.await.map_err(|e| e.to_string())
	}
	/// 注册 RPC handler（只允许注册一次）
	pub async fn register_rpc_handler<F>(self: &Arc<Self>, handler: F)
	where
		F: FnMut(EventEnvelope<T>) -> T + Send + 'static,
	{
		// 占位接口，避免未使用变量警告
		let _ = handler;
		// let _rpc_rx = self.rpc_tx.clone();
		// let _bus = Arc::clone(self);
		// tokio::spawn(async move {
		//     // ...
		// });
	}
}


