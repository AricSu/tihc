use axum::{
    extract::{Query, State},
    http::{HeaderValue, StatusCode},
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::{get, post},
    Json, Router,
};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use uuid::Uuid;

// RCA 相关数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub session_id: Option<String>,
    pub timestamp: Option<i64>,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: String,
    pub session_id: String,
    pub suggestions: Option<Vec<String>>,
    pub finished: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRequest {
    pub user_id: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub session_id: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamChunk {
    pub session_id: String,
    pub text: String,
    pub finished: bool,
    pub timestamp: i64,
}

// 应用状态
#[derive(Clone)]
pub struct RCAAppState {
    // 这里可以注入 autoflow 插件服务
    // autoflow_service: Arc<dyn AutoflowService>,
}

impl RCAAppState {
    pub fn new() -> Self {
        Self {
            // autoflow_service: autoflow_service,
        }
    }
}

pub struct RCAController;

impl RCAController {
    /// 创建路由
    pub fn routes() -> Router<RCAAppState> {
        Router::new()
            // 传统 REST API
            .route("/api/rca/chat", post(Self::chat))
            .route("/api/rca/session", post(Self::create_session))
            .route("/api/rca/sessions", get(Self::get_sessions))
            .route("/api/rca/history/:session_id", get(Self::get_history))
            // 流式 API
            .route("/api/rca/chat/stream", post(Self::chat_stream))
            .route("/api/rca/chat/sse", get(Self::chat_sse))
            // 健康检查
            .route("/api/rca/health", get(Self::health_check))
    }

    /// 普通聊天接口 (非流式)
    async fn chat(
        State(_state): State<RCAAppState>,
        Json(request): Json<ChatRequest>,
    ) -> Result<Json<ChatResponse>, StatusCode> {
        // 模拟调用 autoflow 插件
        let session_id = request.session_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        
        // 这里应该调用 plugin_autoflow_client
        let response = ChatResponse {
            message: format!("收到消息: {}", request.message),
            session_id,
            suggestions: Some(vec![
                "查看慢查询日志".to_string(),
                "分析连接池状态".to_string(),
                "检查资源使用情况".to_string(),
            ]),
            finished: true,
        };

        Ok(Json(response))
    }

    /// 流式聊天接口 (HTTP Stream)
    async fn chat_stream(
        State(_state): State<RCAAppState>,
        Json(request): Json<ChatRequest>,
    ) -> Result<axum::response::Response, StatusCode> {
        let session_id = request.session_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        
        // 创建流式响应
        let stream = Self::create_chat_stream(session_id.clone(), request.message).await;
        
        let body = axum::body::Body::from_stream(stream);
        
        Ok(axum::response::Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/x-ndjson") // Newline Delimited JSON
            .header("Cache-Control", "no-cache")
            .header("Connection", "keep-alive")
            .header("X-Accel-Buffering", "no") // Nginx 禁用缓冲
            .body(body)
            .unwrap())
    }

    /// SSE 流式接口
    async fn chat_sse(
        State(_state): State<RCAAppState>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
        let session_id = params
            .get("session_id")
            .cloned()
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        
        let message = params
            .get("message")
            .cloned()
            .unwrap_or_else(|| "Hello".to_string());

        let stream = Self::create_sse_stream(session_id, message).await;

        Sse::new(stream).keep_alive(KeepAlive::default())
    }

    /// 创建聊天流
    async fn create_chat_stream(
        session_id: String,
        message: String,
    ) -> impl Stream<Item = Result<bytes::Bytes, std::io::Error>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        
        // 模拟流式 AI 响应
        tokio::spawn(async move {
            let words = vec![
                "正在",
                "分析",
                "您的",
                "数据库",
                "问题",
                "...\n\n",
                "发现",
                "可能的",
                "根因",
                ":\n",
                "1. ",
                "连接池",
                "配置",
                "不当",
                "\n",
                "2. ",
                "慢查询",
                "较多",
                "\n",
                "3. ",
                "索引",
                "缺失",
                "\n\n",
                "建议",
                "您",
                "检查",
                "相关",
                "配置",
                "。"
            ];

            for (i, word) in words.iter().enumerate() {
                let chunk = StreamChunk {
                    session_id: session_id.clone(),
                    text: word.to_string(),
                    finished: i == words.len() - 1,
                    timestamp: chrono::Utc::now().timestamp_millis(),
                };

                let json_line = serde_json::to_string(&chunk).unwrap() + "\n";
                let bytes = bytes::Bytes::from(json_line);
                
                if tx.send(Ok(bytes)).is_err() {
                    break;
                }

                // 模拟 AI 生成延迟
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        UnboundedReceiverStream::new(rx)
    }

    /// 创建 SSE 流
    async fn create_sse_stream(
        session_id: String,
        message: String,
    ) -> impl Stream<Item = Result<Event, axum::Error>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        
        tokio::spawn(async move {
            let words = vec![
                "开始", "分析", "数据库", "性能", "问题", "...", 
                "检测", "到", "异常", "指标", ":", 
                "CPU", "使用率", "过高", "，",
                "建议", "优化", "查询", "语句", "。"
            ];

            for (i, word) in words.iter().enumerate() {
                let chunk = StreamChunk {
                    session_id: session_id.clone(),
                    text: format!("{} ", word),
                    finished: i == words.len() - 1,
                    timestamp: chrono::Utc::now().timestamp_millis(),
                };

                let event = Event::default()
                    .event("message")
                    .data(serde_json::to_string(&chunk).unwrap());

                if tx.send(Ok(event)).is_err() {
                    break;
                }

                tokio::time::sleep(Duration::from_millis(150)).await;
            }

            // 发送结束事件
            let end_event = Event::default()
                .event("end")
                .data(serde_json::json!({"session_id": session_id, "finished": true}).to_string());
            
            let _ = tx.send(Ok(end_event));
        });

        UnboundedReceiverStream::new(rx)
    }

    /// 创建会话
    async fn create_session(
        State(_state): State<RCAAppState>,
        Json(_request): Json<SessionRequest>,
    ) -> Result<Json<SessionResponse>, StatusCode> {
        let response = SessionResponse {
            session_id: Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok(Json(response))
    }

    /// 获取会话列表
    async fn get_sessions(
        State(_state): State<RCAAppState>,
    ) -> Result<Json<Vec<SessionResponse>>, StatusCode> {
        // 模拟返回会话列表
        let sessions = vec![
            SessionResponse {
                session_id: Uuid::new_v4().to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
            }
        ];

        Ok(Json(sessions))
    }

    /// 获取会话历史
    async fn get_history(
        State(_state): State<RCAAppState>,
        axum::extract::Path(session_id): axum::extract::Path<String>,
    ) -> Result<Json<Vec<ChatResponse>>, StatusCode> {
        // 模拟返回历史消息
        let history = vec![
            ChatResponse {
                message: "您好，我是 TiHC RCA 助手。".to_string(),
                session_id: session_id.clone(),
                suggestions: None,
                finished: true,
            }
        ];

        Ok(Json(history))
    }

    /// 健康检查
    async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
        Ok(Json(serde_json::json!({
            "status": "healthy",
            "service": "rca_controller",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "features": {
                "sse": true,
                "http_stream": true,
                "websocket": false
            }
        })))
    }
}