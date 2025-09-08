// HTTP Controllers

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::application::services::{
    EditorApplicationService, CreateQueryRequest, ExecuteQueryRequest, UpdateQueryRequest,
};
use crate::domain::shared::Pagination;
use crate::interface::http::responses::{ApiResponse, ErrorResponse};

/// 编辑器控制器
pub struct EditorController;

impl EditorController {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/queries", post(Self::create_query))
            .route("/queries/{id}", get(Self::get_query))
            .route("/queries/{id}", put(Self::update_query))
            .route("/queries/{id}", delete(Self::delete_query))
            .route("/queries/{id}/execute", post(Self::execute_query))
            .route("/databases/{database_id}/queries", get(Self::list_queries))
    }
    
    async fn create_query(
        State(state): State<AppState>,
        Json(request): Json<CreateQueryRequest>,
    ) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
        match state.editor_service.create_query(request).await {
            Ok(query) => Ok(Json(ApiResponse::success(json!(query)))),
            Err(error) => Err(Self::handle_error(error)),
        }
    }
    
    async fn get_query(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
        match state.editor_service.get_query(&id).await {
            Ok(query) => Ok(Json(ApiResponse::success(json!(query)))),
            Err(error) => Err(Self::handle_error(error)),
        }
    }
    
    async fn update_query(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Json(mut request): Json<UpdateQueryRequest>,
    ) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
        request.query_id = id;
        match state.editor_service.update_query(request).await {
            Ok(query) => Ok(Json(ApiResponse::success(json!(query)))),
            Err(error) => Err(Self::handle_error(error)),
        }
    }
    
    async fn delete_query(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
        match state.editor_service.delete_query(&id).await {
            Ok(_) => Ok(Json(ApiResponse::success(json!({"message": "查询已删除"})))),
            Err(error) => Err(Self::handle_error(error)),
        }
    }
    
    async fn execute_query(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Json(mut request): Json<ExecuteQueryRequest>,
    ) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
        request.query_id = id;
        match state.editor_service.execute_query(request).await {
            Ok(result) => Ok(Json(ApiResponse::success(json!(result)))),
            Err(error) => Err(Self::handle_error(error)),
        }
    }
    
    async fn list_queries(
        State(state): State<AppState>,
        Path(database_id): Path<String>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ErrorResponse>)> {
        match state.editor_service.list_queries(&database_id, pagination).await {
            Ok(queries) => Ok(Json(ApiResponse::success(json!(queries)))),
            Err(error) => Err(Self::handle_error(error)),
        }
    }
    
    fn handle_error(error: crate::domain::shared::DomainError) -> (StatusCode, Json<ErrorResponse>) {
        use crate::domain::shared::DomainError;
        
        let (status, message) = match error {
            DomainError::NotFound { resource } => (StatusCode::NOT_FOUND, format!("资源未找到: {}", resource)),
            DomainError::ValidationError { message } => (StatusCode::BAD_REQUEST, message),
            DomainError::BusinessRuleViolation { rule } => (StatusCode::UNPROCESSABLE_ENTITY, rule),
            DomainError::ExternalServiceError { service } => (StatusCode::BAD_GATEWAY, format!("外部服务错误: {}", service)),
            DomainError::InternalError { message } => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };
        
        (status, Json(ErrorResponse::new(message)))
    }
}

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub editor_service: Arc<dyn EditorApplicationService>,
    // 其他服务...
}

impl AppState {
    pub fn new(editor_service: Arc<dyn EditorApplicationService>) -> Self {
        Self { editor_service }
    }
}
