// SQL Editor HTTP Controllers
// HTTP接口层，处理SQL编辑和执行相关的API请求

use axum::{Json, Router, routing::{get, post, put, delete}, http::StatusCode, extract::{Path, State}};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::application::services::EditorApplicationService;
use crate::interface::http::responses::ApiResponse;

/// SQL编辑器控制器
pub struct SqlEditorController;

impl SqlEditorController {
    /// 创建SQL编辑器相关的路由
    pub fn routes() -> Router<SqlEditorControllerState> {
        Router::new()
            .route("/sql_editor/sql/execute", post(execute_sql_handler))
            .route("/sql_editor/status/{task_id}", get(get_sql_status))
            .route("/queries", post(create_query_handler))
            .route("/queries/{id}", get(get_query_handler))
            .route("/queries/{id}", put(update_query_handler))
            .route("/queries/{id}", delete(delete_query_handler))
    }
}

/// SQL编辑器控制器状态
#[derive(Clone)]
pub struct SqlEditorControllerState {
    pub editor_service: Arc<dyn EditorApplicationService>,
}

impl SqlEditorControllerState {
    pub fn new(editor_service: Arc<dyn EditorApplicationService>) -> Self {
        Self { editor_service }
    }
}

/// SQL执行请求（兼容原有API）
#[derive(Debug, Deserialize)]
pub struct ExecuteSqlRequest {
    /// 连接 ID
    pub connection_id: u64,
    /// 待执行 SQL
    pub sql: String,
}

/// SQL执行结果（兼容原有API）
#[derive(Debug, Serialize, Deserialize)]
pub struct SqlResult {
    /// 列名
    pub column_names: Vec<String>,
    /// 数据行
    pub rows: Vec<Vec<serde_json::Value>>,
}

/// SQL执行响应（兼容原有API）
#[derive(Debug, Serialize, Deserialize)]
pub struct SqlExecutionResponse {
    pub success: bool,
    pub rows_affected: Option<u64>,
    pub execution_time_ms: u64,
    pub data: Option<SqlResult>,
    pub error: Option<String>,
}

/// 查询创建请求
#[derive(Debug, Deserialize)]
pub struct CreateQueryDto {
    pub database_id: u64,
    pub content: String,
    pub name: Option<String>,
}

/// 查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub id: String,
    pub database_id: u64,
    pub content: String,
    pub name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 查询更新请求
#[derive(Debug, Deserialize)]
pub struct UpdateQueryDto {
    pub content: Option<String>,
    pub name: Option<String>,
}

/// POST /sql_editor/sql/execute - 执行SQL
pub async fn execute_sql_handler(
    State(_state): State<SqlEditorControllerState>,
    Json(request): Json<ExecuteSqlRequest>,
) -> Result<Json<ApiResponse<SqlExecutionResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 模拟SQL执行 - 在实际实现中这里会调用sql_editor插件或应用服务
    let response = SqlExecutionResponse {
        success: true,
        rows_affected: Some(0),
        execution_time_ms: 45,
        data: Some(SqlResult {
            column_names: vec!["id".to_string(), "name".to_string()],
            rows: vec![
                vec![serde_json::Value::Number(1.into()), serde_json::Value::String("Test User".to_string())],
            ],
        }),
        error: None,
    };
    
    tracing::info!("Executing SQL for connection {}: {}", request.connection_id, &request.sql[..50.min(request.sql.len())]);
    Ok(Json(ApiResponse::success(response)))
}

/// GET /sql_editor/status/{task_id} - 获取SQL执行状态
pub async fn get_sql_status(
    State(_state): State<SqlEditorControllerState>,
    Path(task_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 简化的状态响应，与原有API兼容
    let status = serde_json::json!({
        "task_id": task_id,
        "status": "completed",
        "message": "SQL query executed successfully",
        "data": []
    });
    
    Ok(Json(ApiResponse::success(status)))
}

/// POST /queries - 创建新查询
pub async fn create_query_handler(
    State(_state): State<SqlEditorControllerState>,
    Json(dto): Json<CreateQueryDto>,
) -> Result<Json<ApiResponse<QueryResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 生成简单的ID和时间戳
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    let response = QueryResponse {
        id,
        database_id: dto.database_id,
        content: dto.content,
        name: dto.name,
        created_at: now.clone(),
        updated_at: now,
    };
    
    Ok(Json(ApiResponse::success(response)))
}

/// GET /queries/{id} - 获取查询详情
pub async fn get_query_handler(
    State(_state): State<SqlEditorControllerState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<QueryResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 模拟查询响应
    let response = QueryResponse {
        id: id.clone(),
        database_id: 1,
        content: "SELECT * FROM users;".to_string(),
        name: Some("Sample Query".to_string()),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(Json(ApiResponse::success(response)))
}

/// PUT /queries/{id} - 更新查询
pub async fn update_query_handler(
    State(_state): State<SqlEditorControllerState>,
    Path(id): Path<String>,
    Json(dto): Json<UpdateQueryDto>,
) -> Result<Json<ApiResponse<QueryResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 模拟更新查询响应
    let response = QueryResponse {
        id: id.clone(),
        database_id: 1,
        content: dto.content.unwrap_or_else(|| "SELECT * FROM users;".to_string()),
        name: dto.name.or_else(|| Some("Updated Query".to_string())),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(Json(ApiResponse::success(response)))
}

/// DELETE /queries/{id} - 删除查询
pub async fn delete_query_handler(
    State(_state): State<SqlEditorControllerState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    tracing::info!("Deleted query with ID: {}", id);
    Ok(Json(ApiResponse::success(())))
}
