use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use std::sync::Arc;

use crate::application::table::{
    TableApplicationService, TableApplicationServiceImpl, 
    TableListResponse, TableDetailResponse, ColumnOperationResponse
};
use crate::domain::table::{TableId, AddColumnRequest};
use crate::interface::http::responses::ApiResponse;

/// Controller for handling table-related HTTP endpoints
pub struct TableController;

impl TableController {
    /// Creates the table router with all endpoints
    pub fn routes() -> Router<TableAppState> {
        Router::new()
            .route("/sql_editor/tables", get(list_tables_handler))
            .route("/sql_editor/tables/{table_name}", get(get_table_details_handler))
            .route("/sql_editor/tables/{table_name}/columns", post(add_column_handler))
            .route("/sql_editor/tables/{table_name}/columns/{column_name}", delete(delete_column_handler))
    }
}

/// 表应用状态
#[derive(Clone)]
pub struct TableAppState {
    pub table_service: Arc<dyn TableApplicationService>,
}

impl TableAppState {
    pub fn new(table_service: Arc<dyn TableApplicationService>) -> Self {
        Self { table_service }
    }
}

/// Handler for listing all tables
async fn list_tables_handler(
    State(state): State<TableAppState>,
) -> Result<Json<ApiResponse<TableListResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    match state.table_service.list_tables(None).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => {
            tracing::error!("Failed to list tables: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::empty_error("Failed to list tables", 500))
            ))
        }
    }
}

/// Handler for getting table details
async fn get_table_details_handler(
    Path(table_name): Path<String>,
    State(state): State<TableAppState>,
) -> Result<Json<ApiResponse<TableDetailResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let table_id = TableId::new(table_name);
    
    match state.table_service.get_table_details(&table_id).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => {
            tracing::error!("Failed to get table details: {}", e);
            if e.to_string().contains("not found") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::empty_error("Table not found", 404))
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::empty_error("Failed to get table details", 500))
                ))
            }
        }
    }
}

/// Handler for adding a column to a table
async fn add_column_handler(
    Path(table_name): Path<String>,
    State(state): State<TableAppState>,
    Json(request): Json<AddColumnRequest>,
) -> Result<Json<ApiResponse<ColumnOperationResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let table_id = TableId::new(table_name);
    
    // Validate request first
    if let Err(e) = request.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::empty_error(&format!("Invalid request: {}", e), 400))
        ));
    }
    
    match state.table_service.add_column(&table_id, request).await {
        Ok(response) => {
            if response.status == "success" {
                Ok(Json(ApiResponse::success(response)))
            } else {
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::empty_error(&response.message, 400))
                ))
            }
        }
        Err(e) => {
            tracing::error!("Failed to add column: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::empty_error("Failed to add column", 500))
            ))
        }
    }
}

/// Handler for deleting a column from a table
async fn delete_column_handler(
    Path((table_name, column_name)): Path<(String, String)>,
    State(state): State<TableAppState>,
) -> Result<Json<ApiResponse<ColumnOperationResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let table_id = TableId::new(table_name);
    
    match state.table_service.remove_column(&table_id, &column_name).await {
        Ok(response) => {
            if response.status == "success" {
                Ok(Json(ApiResponse::success(response)))
            } else {
                if response.message.contains("not found") {
                    Err((
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::empty_error(&response.message, 404))
                    ))
                } else {
                    Err((
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::empty_error(&response.message, 400))
                    ))
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete column: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::empty_error("Failed to delete column", 500))
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    fn create_test_app_state() -> TableAppState {
        let table_service: Arc<dyn TableApplicationService> = Arc::new(TableApplicationServiceImpl::new());
        TableAppState::new(table_service)
    }

    #[tokio::test]
    async fn test_list_tables_endpoint() {
        let app_state = create_test_app_state();
        let app = TableController::routes().with_state(app_state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/sql_editor/tables")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_table_details_endpoint() {
        let app_state = create_test_app_state();
        let app = TableController::routes().with_state(app_state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/sql_editor/tables/users")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_nonexistent_table_details() {
        let app_state = create_test_app_state();
        let app = TableController::routes().with_state(app_state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/sql_editor/tables/nonexistent")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_add_column_endpoint() {
        let app_state = create_test_app_state();
        let app = TableController::routes().with_state(app_state);

        let request_body = serde_json::json!({
            "column_name": "age",
            "column_type": "int",
            "nullable": true
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/sql_editor/tables/users/columns")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_add_column_invalid_request() {
        let app_state = create_test_app_state();
        let app = TableController::routes().with_state(app_state);

        let request_body = serde_json::json!({
            "column_name": "",
            "column_type": "int"
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/sql_editor/tables/users/columns")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_column_endpoint() {
        let app_state = create_test_app_state();
        let app = TableController::routes().with_state(app_state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/sql_editor/tables/users/columns/email")
                    .method("DELETE")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_column() {
        let app_state = create_test_app_state();
        let app = TableController::routes().with_state(app_state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/sql_editor/tables/users/columns/nonexistent")
                    .method("DELETE")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
