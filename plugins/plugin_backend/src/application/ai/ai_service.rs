use anyhow::Result;
use async_stream::stream;
use futures::stream::Stream;
use futures::{StreamExt};
use microkernel::event_bus::EventBus;
use microkernel::plugin::PluginEvent;
use serde_json::Value;
use std::pin::Pin;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct BusMessage {
    pub data: Value,
}

/// AI 服务 - 通过消息总线与 Autoflow 插件通信
use std::sync::Arc;

pub struct AiService {
    event_bus: Arc<EventBus<PluginEvent>>,
}

impl AiService {
    pub fn new(event_bus: Arc<EventBus<PluginEvent>>) -> Self {
        Self {
            event_bus,
        }
    }

    pub async fn send_chat_message(
        &self,
        session_id: String,
        message: String,
        user_id: Option<i64>,
    ) -> Result<AiChatResponse> {
        debug!("Sending chat message for session: {}", session_id);

        let _request = AiChatRequest {
            session_id: session_id.clone(),
            message,
            user_id,
            chat_id: None,
        };
        let mut response_stream = futures::stream::empty::<Result<BusMessage, anyhow::Error>>(); // 占位，需实现总线客户端

        // 收集所有流式响应块，组合成完整响应
        let mut full_text = String::new();
        let mut chat_id = None;
        let mut session_id_result = session_id;

        while let Some(chunk_result) = response_stream.next().await {
            match chunk_result {
                Ok(bus_msg) => {
                    match serde_json::from_value::<AiChatChunk>(bus_msg.data) {
                        Ok(chunk) => {
                            full_text.push_str(&chunk.text);
                            if chunk.chat_id.is_some() {
                                chat_id = chunk.chat_id;
                            }
                            // session_id 可选，缺失时用原始 session_id
                            if let Some(sid) = chunk.session_id {
                                session_id_result = sid;
                            }
                            // 如果这是最后一个块，跳出循环
                            if chunk.finished {
                                break;
                            }
                        }
                        Err(e) => {
                            return Err(anyhow::anyhow!("Failed to deserialize chat chunk: {}", e));
                        }
                    }
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Stream error: {}", e));
                }
            }
        }

        let ai_response = AiChatResponse {
            session_id: session_id_result,
            chat_id,
            text: full_text,
            finished: true,
            error: None,
        };

        Ok(ai_response)
    }

    /// 发送流式聊天请求（返回真正的流）
    pub async fn send_chat_stream(
        &self,
        session_id: String,
        message: String,
        user_id: Option<i64>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<AiChatChunk>> + Send>>> {
        debug!(
            "🚀 [AI_SERVICE] Sending chat stream for session: {}, message: {}",
            session_id, message
        );
        tracing::trace!(target: "ai_service", "[AI_SERVICE] send_chat_stream loop start");

        let request = AiChatRequest {
            session_id,
            message,
            user_id,
            chat_id: None,
        };

        // 广播请求事件（DDD：应用层只负责分发，领域模型独立）
        let req_value = serde_json::to_value(&request).unwrap_or_default();
        let event = PluginEvent::Custom("ai.chat_stream_request".to_string(), req_value);
        let envelope = microkernel::event_bus::EventEnvelope::new(
            "ai.chat_stream_request",
            event,
            None,
        );
        tracing::debug!(target: "ai_service", "[AI_SERVICE] Broadcast chat_stream_request: session_id={}, user_id={:?}, message_len={}", request.session_id, request.user_id, request.message.len());
        let broadcast_result = self.event_bus.broadcast(envelope);
        tracing::debug!(target: "ai_service", "[AI_SERVICE] Broadcast result: {:?}", broadcast_result);

        // 订阅流式响应（DDD：应用层只负责消费，领域模型独立）
        let mut receiver = self.event_bus.subscribe();
        tracing::debug!(target: "ai_service", "[AI_SERVICE] Subscribed to EventBus for ai.chat_stream, receiver={:p}", &receiver);
        let s = stream! {
            tracing::debug!(target: "ai_service", "[AI_SERVICE] Stream loop start");
            loop {
                tracing::debug!(target: "ai_service", "[AI_SERVICE] Waiting for EventBus event...");
                match receiver.recv().await {
                    Ok(event) => {
                        tracing::debug!(target: "ai_service", "[AI_SERVICE] Received event: {}", event.event_type);
                        // 只处理 ai.chat_stream topic
                        if event.event_type == "ai.chat_stream" {
                            tracing::debug!(target: "ai_service", "[AI_SERVICE] Event type matches ai.chat_stream");
                            match &event.payload {
                                PluginEvent::Custom(_, value) => {
                                    tracing::debug!(target: "ai_service", "[AI_SERVICE] PluginEvent::Custom received, value={:?}", value);
                                    // 兼容 Value::Array [topic, obj] 和 Value::Object
                                    let chunk_val = if let Some(arr) = value.as_array() {
                                        if arr.len() == 2 {
                                            arr[1].clone()
                                        } else {
                                            value.clone()
                                        }
                                    } else {
                                        value.clone()
                                    };
                                    match serde_json::from_value::<AiChatChunk>(chunk_val.clone()) {
                                        Ok(chunk) => {
                                            tracing::info!(target: "ai_service", "[AI_SERVICE] Yield chunk: session_id={:?}, text_len={}, finished={}, chat_id={:?}", chunk.session_id, chunk.text.len(), chunk.finished, chunk.chat_id);
                                            let is_finished = chunk.finished;
                                            yield Ok(chunk);
                                            if is_finished {
                                                tracing::info!(target: "ai_service", "[AI_SERVICE] Chunk finished, breaking loop");
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            let raw_json = serde_json::to_string(&chunk_val).unwrap_or_default();
                                            tracing::error!(target: "ai_service", "[AI_SERVICE] Failed to deserialize AiChatChunk: {:?}, raw_json={}", e, raw_json);
                                        }
                                    }
                                }
                                other => {
                                    tracing::warn!(target: "ai_service", "[AI_SERVICE] Unexpected PluginEvent payload: {:?}", other);
                                }
                            }
                        } else {
                            tracing::debug!(target: "ai_service", "[AI_SERVICE] Ignored event type: {}", event.event_type);
                        }
                    }
                    Err(e) => {
                        tracing::error!(target: "ai_service", "[AI_SERVICE] EventBus recv error: {:?}", e);
                        break;
                    }
                }
            }
            tracing::debug!(target: "ai_service", "[AI_SERVICE] Stream loop end");
        };
        Ok(Box::pin(s))
    }
}

// 重新导出消息类型以便在 backend 中使用
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatRequest {
    pub session_id: String,
    pub message: String,
    pub user_id: Option<i64>,
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatResponse {
    pub session_id: String,
    pub chat_id: Option<String>,
    pub text: String,
    pub finished: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatChunk {
    pub session_id: Option<String>,
    pub chat_id: Option<String>,
    pub text: String,
    pub finished: bool,
}
