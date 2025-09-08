// DDL Precheck HTTP Controllers
// HTTP接口层，处理DDL预检查相关的API请求

use axum::{Json, Router, routing::post, http::StatusCode, extract::State};
use std::sync::Arc;
use crate::application::ddl_precheck::{
    DDLPrecheckApplicationService, DDLPrecheckRequest, DDLPrecheckResponse
};
use crate::interface::http::responses::ApiResponse;

/// DDL预检查控制器
pub struct DDLPrecheckController;

impl DDLPrecheckController {
    /// 创建DDL预检查相关的路由
    pub fn routes() -> Router<DDLPrecheckControllerState> {
        Router::new()
            .route("/ddl/precheck", post(ddl_precheck_handler))
    }
}

/// DDL预检查控制器状态
#[derive(Clone)]
pub struct DDLPrecheckControllerState {
    pub ddl_service: Arc<dyn DDLPrecheckApplicationService>,
}

impl DDLPrecheckControllerState {
    pub fn new(ddl_service: Arc<dyn DDLPrecheckApplicationService>) -> Self {
        Self { ddl_service }
    }
}

/// 处理 DDL 预检查请求
/// POST /ddl/precheck - 执行DDL语句预检查
pub async fn ddl_precheck_handler(
    State(state): State<DDLPrecheckControllerState>,
    Json(request): Json<DDLPrecheckRequest>,
) -> Result<Json<ApiResponse<DDLPrecheckResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    tracing::info!(target: "ddl_controller", "Processing DDL precheck request");
    
    match state.ddl_service.precheck_ddl(request).await {
        Ok(response) => {
            tracing::info!(target: "ddl_controller", "DDL precheck completed successfully");
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            let error_msg = e.to_string();
            tracing::error!(target: "ddl_controller", "DDL precheck failed: {}", error_msg);
            
            let status_code = match e {
                crate::domain::shared::DomainError::ValidationError { .. } => StatusCode::BAD_REQUEST,
                crate::domain::shared::DomainError::ExternalServiceError { .. } => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            
            let error_code = match status_code {
                StatusCode::BAD_REQUEST => 400,
                StatusCode::SERVICE_UNAVAILABLE => 503,
                _ => 500,
            };
            
            Err((status_code, Json(ApiResponse::empty_error(&error_msg, error_code))))
        }
    }
}
