use futures::StreamExt;
use std::sync::Arc;
use tracing::info;

use axum::{extract::Request, response::Response};
use microkernel::{
    event_bus::{EventBus, EventEnvelope},
    plugin::{KernelPlugin, PluginEvent, PluginHandler, PluginRegistry, RegisterHttpRoute},
};

use crate::application::AutoflowSessionService;
use crate::domain::{AiChatRequest, AutoflowConfig};

use microkernel::plugin::PluginFactory;

pub fn autoflow_plugin_factory() -> Box<dyn microkernel::plugin::KernelPlugin> {
    Box::new(AutoflowPlugin::default())
}

inventory::submit! {
    PluginFactory(autoflow_plugin_factory)
}

/// Autoflow AI 插件
/// 负责处理来自 backend 的 AI 聊天请求
pub struct AutoflowPlugin {
    // name: String,
    // description: String,
    service: Arc<AutoflowSessionService>,
}

impl AutoflowPlugin {
    /// 创建新的 Autoflow 插件实例
    pub fn new(config: AutoflowConfig) -> Self {
        let service = Arc::new(AutoflowSessionService::new(config));
        Self { service }
    }

    pub fn default() -> Self {
        Self::new(crate::domain::AutoflowConfig::default())
    }

    /// 统一处理 chat_stream，支持流式广播和收集完整响应
    async fn process_chat_stream(
        service: Arc<AutoflowSessionService>,
        request: &AiChatRequest,
        mut on_chunk: impl FnMut(&crate::domain::chat::StreamChunk) + Send,
    ) -> (String, Option<String>, bool, Option<String>) {
        tracing::info!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] process_chat_stream called: session_id={}, message={}", request.session_id, request.message);
        tracing::trace!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] process_chat_stream loop start");
        let mut full_text = String::new();
        let mut chat_id = None;
        let mut finished = false;
        let mut error_msg = None;
        match service
            .send_message(Some(&request.session_id), &request.message)
            .await
        {
            Ok(mut stream) => {
                tracing::info!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] service.send_message success, streaming...");
                while let Some(chunk_result) = stream.next().await {
                    match chunk_result {
                        Ok(chunk) => {
                            tracing::info!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] Got chunk: len={}, finished={}", chunk.text.len(), chunk.finished);
                            full_text.push_str(&chunk.text);
                            if chunk.chat_id.is_some() {
                                chat_id = chunk.chat_id.clone();
                            }
                            on_chunk(&chunk);
                            if chunk.finished {
                                finished = true;
                                break;
                            }
                        }
                        Err(e) => {
                            tracing::error!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] Chunk error: {}", e);
                            error_msg = Some(e.to_string());
                            finished = true;
                            let err_chunk = crate::domain::chat::StreamChunk {
                                text: format!("[autoflow error]: {}", e),
                                finished: true,
                                chat_id: None,
                                chat_name: None,
                            };
                            on_chunk(&err_chunk);
                            break;
                        }
                    }
                }
                tracing::trace!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] process_chat_stream loop end");
            }
            Err(e) => {
                tracing::error!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] service.send_message failed: {}", e);
                error_msg = Some("Chat request failed".to_string());
                finished = true;
                let err_chunk = crate::domain::chat::StreamChunk {
                    text: "[autoflow error]: Chat request failed".to_string(),
                    finished: true,
                    chat_id: None,
                    chat_name: None,
                };
                on_chunk(&err_chunk);
            }
        }
        (full_text, chat_id, finished, error_msg)
    }

    /// 注册 PluginEvent 事件处理器
    pub fn register_handlers(
        &self,
        bus: Arc<EventBus<PluginEvent>>,
        registry: Arc<PluginRegistry>,
    ) {
        info!("📋 [AUTOFLOW_PLUGIN] Registering /chat/stream route...");
        // 注册 /chat/stream 路由
        let handler: PluginHandler = Arc::new(move |req: Request<axum::body::Body>| {
            Box::pin(async move {
                Response::new(axum::body::Body::from(
                    "[autoflow_client] chat stream response",
                ))
            })
        });
        let path = "/chat/stream".to_string();
        registry.routes.insert(path.clone(), handler.clone());
        // 广播路由注册事件
        let event = EventEnvelope::new(
            "plugin_register_http_route",
            PluginEvent::RegisterHttpRoute(RegisterHttpRoute { path }),
            None,
        );
        let _ = bus.broadcast(event);
        info!("✅ [AUTOFLOW_PLUGIN] /chat/stream route registered!");

        // 订阅 ai.chat_stream 请求事件，实现流式 chunk 分发
        let bus_stream = bus.subscribe();
        let service = self.service.clone();
        let bus_clone = bus.clone();
        tokio::spawn(async move {
            let mut receiver = bus_stream;
            tracing::info!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] EventBus subscription started");
            tracing::trace!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] EventBus main loop start");
            loop {
                tracing::trace!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] Waiting for EventBus event...");
                match receiver.recv().await {
                    Ok(event) => {
                        tracing::info!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] Received event: {}", event.event_type);
                        if event.event_type == "ai.chat_stream_request" {
                            // 兼容 Custom payload 结构，提取第二个元素
                            let ai_req_val = match &event.payload {
                                PluginEvent::Custom(_, v) => {
                                    // v 是 Value::Array [topic, obj]
                                    if let Some(arr) = v.as_array() {
                                        if arr.len() == 2 {
                                            arr[1].clone()
                                        } else {
                                            v.clone()
                                        }
                                    } else {
                                        v.clone()
                                    }
                                }
                                _ => serde_json::to_value(&event.payload).unwrap_or_default()
                            };
                            match serde_json::from_value::<AiChatRequest>(ai_req_val.clone()) {
                                Ok(mut request) => {
                                    // Verify session_id
                                    if request.session_id.trim().is_empty() {
                                        request.session_id = service.create_session_id().await;
                                    }
                                    match AutoflowPlugin::process_chat_stream(
                                        service.clone(),
                                        &request,
                                        |chunk| {
                                            tracing::info!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] on_chunk: text_len={}, finished={}, chat_id={:?}", chunk.text.len(), chunk.finished, chunk.chat_id);
                                            let chunk_value = serde_json::to_value(chunk).unwrap_or_default();
                                            let event_payload = PluginEvent::Custom("ai.chat_stream".to_string(), chunk_value);
                                            let chunk_event = EventEnvelope::new(
                                                "ai.chat_stream",
                                                event_payload,
                                                None,
                                            );
                                            tracing::info!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] Broadcasting chunk event: finished={}, text={}", chunk.finished, chunk.text);
                                            let _ = bus_clone.broadcast(chunk_event);
                                        }
                                    ).await {
                                        (full_text, chat_id, finished, error_msg) => {
                                            tracing::info!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] process_chat_stream finished: chat_id={:?}, full_text_len={}, finished={}, error_msg={:?}", chat_id, full_text.len(), finished, error_msg);
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] Failed to parse AiChatRequest: {} | Payload: {:?}", e, ai_req_val);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(target: "plugin_autoflow_client::plugin", "[AUTOFLOW_PLUGIN] EventBus recv error: {}", e);
                    }
                }
            }
        });
    }
}

impl KernelPlugin for AutoflowPlugin {
    fn register(&self, bus: Arc<EventBus<PluginEvent>>, registry: Arc<PluginRegistry>) {
        self.register_handlers(bus, registry);
    }
}
