use axum::{
    Json,
    extract::{Query, State},
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures::StreamExt;
use serde::Deserialize;
use serde_json::json;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::application::chat::ChatHistoryService;
use crate::domain::chat::{ChatHistoryRequest, CreateChatSessionRequest, ListChatSessionsRequest};
use crate::infrastructure::startup::InfraState;
use crate::interface::http::response::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct ChatHistoryPayload {
    #[serde(alias = "userId")]
    pub user_id: i64,
    #[serde(alias = "sessionId")]
    pub session_id: Option<i64>,
    #[serde(alias = "userMessage", alias = "user")]
    pub user_message: String,
    #[serde(alias = "assistantMessage")]
    pub assistant_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StreamChatQuery {
    pub messages: String,
    #[serde(alias = "userId")]
    pub user_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct IncomingMessage {
    pub role: String,
    pub content: String,
}

pub async fn get_chat_history_handler(
    State(state): State<Arc<InfraState>>,
    Query(request): Query<ChatHistoryRequest>,
) -> impl IntoResponse {
    let chat_service = ChatHistoryService::new(
        state.chat_history_repo.clone(),
        state.ai_service.clone(),
    );
    match chat_service.get_chat_history(request).await {
        Ok(histories) => ApiResponse::success(histories).into_response(),
        Err(e) => ApiResponse::<()>::error(500, e.to_string()).into_response(),
    }
}

pub async fn add_chat_message_handler(
    State(state): State<Arc<InfraState>>,
    Json(payload): Json<ChatHistoryPayload>,
) -> impl IntoResponse {
    let chat_service = ChatHistoryService::new(
        state.chat_history_repo.clone(),
        state.ai_service.clone(),
    );
    
    tracing::info!("📝 [HTTP] add_chat_message: user_id={}, session_id={:?}, has_assistant_msg={}", 
        payload.user_id, payload.session_id, payload.assistant_message.is_some());
    
    match payload.assistant_message {
        // 如果前端已经有了助手回复（来自流式API），直接保存
        Some(assistant_message) => {
            match chat_service
                .add_chat_message(
                    payload.user_id,
                    payload.session_id,
                    payload.user_message,
                    assistant_message,
                )
                .await
            {
                Ok(chat_history) => {
                    tracing::debug!("✅ [HTTP] Chat history saved: id={}", chat_history.id);
                    ApiResponse::success(chat_history).into_response()
                }
                Err(e) => {
                    tracing::error!("❌ [HTTP] Failed to save chat history: {}", e);
                    ApiResponse::<()>::error(500, e.to_string()).into_response()
                }
            }
        }
        // 如果没有助手回复，使用AI服务获取（用于非流式API）
        None => {
            match chat_service
                .send_ai_message(payload.user_id, payload.session_id, payload.user_message)
                .await
            {
                Ok((chat_history, ai_response)) => {
                    let response = serde_json::json!({
                        "chat_history": chat_history,
                        "ai_response": ai_response
                    });
                    ApiResponse::success(response).into_response()
                }
                Err(e) => ApiResponse::<()>::error(500, e.to_string()).into_response(),
            }
        }
    }
}

pub async fn stream_chat_handler(
    State(state): State<Arc<InfraState>>,
    Query(query): Query<StreamChatQuery>,
) -> Sse<ReceiverStream<Result<Event, Infallible>>> {
    // 校验参数
    if query.messages.trim().is_empty() || query.user_id.is_none() {
        let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(1);
        let error_payload = json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "参数错误：messages 或 userId 不能为空。"
                }
            }],
            "error": "missing messages or userId"
        }).to_string();
        let _ = tx.send(Ok(Event::default().data(error_payload))).await;
        let _ = tx.send(Ok(Event::default().event("complete").data("{}"))).await;
        return Sse::new(ReceiverStream::new(rx));
    }

    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(32);

    // 使用真正的AI服务流式响应
    let ai_service = state.ai_service.clone();
    tokio::spawn(async move {
        // 创建一个临时会话ID（实际项目中应该从请求中获取或创建）
        let session_id = format!("stream_session_{}", chrono::Utc::now().timestamp());
        tracing::debug!("🔧 [HTTP] Starting AI stream for session_id={}", session_id);
        tracing::trace!(target: "chat_controller", "[HTTP] stream_chat_handler loop start");

        match ai_service.send_chat_stream(session_id.clone(), query.messages, query.user_id).await {
            Ok(mut stream) => {
                tracing::debug!("📡 [HTTP] AI stream started for session_id={}", session_id);
                // 处理流式响应
                let mut chunk_idx: usize = 0;
                while let Some(chunk_result) = stream.next().await {
                    chunk_idx += 1;
                    tracing::trace!(target: "chat_controller", "[HTTP] Waiting for next chunk... idx={}", chunk_idx);
                    match chunk_result {
                        Ok(chunk) => {
                            tracing::trace!(target: "chat_controller", "[HTTP] Got chunk: session_id={:?}, chunk_index={}, text_len={}, finished={}", chunk.session_id, chunk_idx, chunk.text.len(), chunk.finished);
                            let payload = json!({
                                "choices": [{
                                    "message": {
                                        "role": "assistant",
                                        "content": chunk.text,
                                    }
                                }],
                                "finished": chunk.finished
                            }).to_string();

                            tracing::trace!(target: "chat_controller", "[HTTP] Sending SSE event: session_id={:?}, chunk_index={}, text_len={}, finished={}", chunk.session_id, chunk_idx, chunk.text.len(), chunk.finished);
                            if tx.send(Ok(Event::default().data(payload))).await.is_err() {
                                tracing::warn!("⚠️ [HTTP] SSE client disconnected for session={}", session_id);
                                break;
                            }

                            // 如果是最后一个块，发送完成事件
                            if chunk.finished {
                                tracing::debug!("✅ [HTTP] session={} stream finished, sending complete event", session_id);
                                let _ = tx
                                    .send(Ok(Event::default().event("complete").data("{}")))
                                    .await;
                                break;
                            }
                        }
                        Err(e) => {
                            tracing::error!("❌ [HTTP] AI stream error for session_id={} err={:?}", session_id, e);
                            let error_payload = json!({
                                "choices": [{
                                    "message": {
                                        "role": "assistant", 
                                        "content": "抱歉，处理您的请求时出现错误。"
                                    }
                                }],
                                "error": e.to_string()
                            }).to_string();
                            
                            let _ = tx.send(Ok(Event::default().data(error_payload))).await;
                            let _ = tx
                                .send(Ok(Event::default().event("complete").data("{}")))
                                .await;
                            break;
                        }
                    }
                }
                tracing::debug!("🔚 [HTTP] AI stream task ended for session_id={}", session_id);
            }
            Err(e) => {
                tracing::error!("Failed to start AI stream: {:?}", e);
                let error_payload = json!({
                    "choices": [{
                        "message": {
                            "role": "assistant",
                            "content": "很抱歉，AI服务当前不可用，请稍后重试。"
                        }
                    }],
                    "error": e.to_string()
                }).to_string();
                
                let _ = tx.send(Ok(Event::default().data(error_payload))).await;
                let _ = tx
                    .send(Ok(Event::default().event("complete").data("{}")))
                    .await;
            }
        }
        tracing::trace!(target: "chat_controller", "[HTTP] stream_chat_handler loop end");
    });

    Sse::new(ReceiverStream::new(rx))
}
/// GET /chat/sessions - 获取用户的会话列表
pub async fn sessions_handler(
    State(state): State<Arc<InfraState>>,
    Query(request): Query<ListChatSessionsRequest>,
) -> impl IntoResponse {
    let chat_service = ChatHistoryService::new(
        state.chat_history_repo.clone(),
        state.ai_service.clone(),
    );

    match chat_service.list_sessions(request.user_id, request.limit).await {
        Ok(sessions) => ApiResponse::success(sessions).into_response(),
        Err(e) => ApiResponse::<()>::error(500, e.to_string()).into_response(),
    }
}

/// POST /chat/sessions - 创建新的聊天会话
pub async fn create_session_handler(
    State(state): State<Arc<InfraState>>,
    Json(request): Json<CreateChatSessionRequest>,
) -> impl IntoResponse {
    let chat_service = ChatHistoryService::new(
        state.chat_history_repo.clone(),
        state.ai_service.clone(),
    );

    match chat_service.create_session(request.user_id, request.title).await {
        Ok(session) => ApiResponse::success(session).into_response(),
        Err(e) => ApiResponse::<()>::error(500, e.to_string()).into_response(),
    }
}

/// 新的聊天开始处理器，支持 user_id 参数
#[derive(Debug, Deserialize)]
pub struct StartChatRequest {
    #[serde(alias = "userId")]
    pub user_id: i64,
    pub messages: Vec<IncomingMessage>,
}

pub async fn start_chat_handler(
    State(state): State<Arc<InfraState>>,
    Json(request): Json<StartChatRequest>,
) -> impl IntoResponse {
    let chat_service = ChatHistoryService::new(
        state.chat_history_repo.clone(),
        state.ai_service.clone(),
    );

    // 提取最新的用户消息
    let user_message = request.messages
        .iter()
        .rev()
        .find(|msg| msg.role.eq_ignore_ascii_case("user"))
        .map(|msg| msg.content.trim().to_string())
        .unwrap_or_else(|| "你好".to_string());

    match chat_service.send_ai_message(request.user_id, None, user_message).await {
        Ok((chat_history, ai_response)) => {
            let response = serde_json::json!({
                "id": format!("chat_{}", chat_history.id),
                "choices": [{
                    "message": {
                        "role": "assistant",
                        "content": ai_response.text
                    }
                }]
            });
            ApiResponse::success(response).into_response()
        }
        Err(e) => ApiResponse::<()>::error(500, e.to_string()).into_response(),
    }
}


