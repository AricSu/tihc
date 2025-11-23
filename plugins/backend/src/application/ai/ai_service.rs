use std::time::Duration;
use std::pin::Pin;
use anyhow::Result;
use futures::{Stream, StreamExt};
use serde_json::json;
use tracing::debug;

// Dummy BusMessage type for placeholder usage
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct BusMessage {
    pub data: Value,
}

/// AI 服务 - 通过消息总线与 Autoflow 插件通信
pub struct AiService {
    // bus_client: BusClient,
    default_timeout: Duration,
}

impl AiService {
    pub fn new() -> Self {
        Self {
            // bus_client: BusClient::new(),
            default_timeout: Duration::from_secs(30),
        }
    }

    /// 发送聊天请求并获取完整响应
    pub async fn send_chat_message(
        &self,
        session_id: String,
        message: String,
        user_id: Option<i64>,
    ) -> Result<AiChatResponse> {
        debug!("Sending chat message for session: {}", session_id);

        let request = AiChatRequest {
            session_id: session_id.clone(),
            message,
            user_id,
            chat_id: None,
        };

        // 获取流式响应并收集完整内容
        // let mut response_stream = self.bus_client.request_stream("ai.chat", json!(request)).await?;
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
                            session_id_result = chunk.session_id;
                            
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
        debug!("🚀 [AI_SERVICE] Sending chat stream for session: {}, message: {}", session_id, message);

        let request = AiChatRequest {
            session_id,
            message,
            user_id,
            chat_id: None,
        };

        debug!("📡 [AI_SERVICE] Requesting stream from message bus topic: ai.chat_stream");
        // let stream = self.bus_client.request_stream("ai.chat_stream", json!(request)).await.map_err(|e| {
        //     tracing::error!("❌ [AI_SERVICE] Failed to request stream from message bus: {}", e);
        //     e
        // })?;
        let stream: futures::stream::Empty<Result<AiChatChunk, anyhow::Error>> = futures::stream::empty(); // 占位
        Ok(Box::pin(stream))
    }

    /// 检查 AI 服务健康状态
    pub async fn health_check(&self) -> Result<HealthCheckResponse> {
        debug!("Checking AI service health");

        let request = HealthCheckRequest {};

        // let response = self.bus_client.send_request("ai.health", json!(request), Some(Duration::from_secs(5))).await?;
        let response = serde_json::json!({"status": "ok"}); // 占位
        let health_response: HealthCheckResponse = serde_json::from_value(response)?;
        Ok(health_response)
    }

    /// 设置请求超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
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
    pub session_id: String,
    pub chat_id: Option<String>,
    pub text: String,
    pub finished: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub message: Option<String>,
}

impl Default for AiService {
    fn default() -> Self {
        Self::new()
    }
}