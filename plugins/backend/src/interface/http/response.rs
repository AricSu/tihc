use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

// 注意：响应 DTO 应该在 interface 层，但避免领域逻辑泄露
// 建议：将业务相关的响应格式移到 application 层的 DTOs 中

/// 统一API响应格式
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "OK".to_string(),
            data: Some(data),
        }
    }

    /// 创建成功响应（无数据）
    pub fn success_without_data() -> ApiResponse<()> {
        ApiResponse {
            code: 200,
            message: "OK".to_string(),
            data: None,
        }
    }

    /// 创建错误响应
    pub fn error(code: u32, message: String) -> ApiResponse<()> {
        ApiResponse {
            code,
            message,
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = if self.code == 200 {
            StatusCode::OK
        } else if self.code >= 400 && self.code < 500 {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };

        (status, Json(self)).into_response()
    }
}

/// 成功响应的便捷宏
#[macro_export]
macro_rules! ok_response {
    ($data:expr) => {
        crate::interface::http::response::ApiResponse::success($data)
    };
    () => {
        crate::interface::http::response::ApiResponse::success_without_data()
    };
}

/// 错误响应的便捷宏
#[macro_export]
macro_rules! error_response {
    ($code:expr, $message:expr) => {
        crate::interface::http::response::ApiResponse::error($code, $message.to_string())
    };
}

pub use error_response;
pub use ok_response;
