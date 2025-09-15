// HTTP Responses

use serde::Serialize;

/// 标准API响应格式
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data,
            message: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl ApiResponse<serde_json::Value> {
    pub fn error(error_message: &str, _code: u16) -> Self {
        Self {
            success: false,
            data: serde_json::Value::Null,
            message: Some(error_message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl ApiResponse<()> {
    pub fn empty_error(error_message: &str, _code: u16) -> Self {
        Self {
            success: false,
            data: (),
            message: Some(error_message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// 错误响应格式
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self {
            success: false,
            error,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// 健康检查响应
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: chrono::Utc::now(),
        }
    }
}
