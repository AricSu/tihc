use axum::{Json, Router, routing::{get, post, delete}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub column_type: String,
}

#[derive(Deserialize)]
pub struct AddColumnRequest {
    pub column_name: String,
    pub column_type: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

async fn list_tables() -> Json<ApiResponse<Vec<Table>>> {
    Json(ApiResponse { data: vec![] })
}

async fn add_column() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "message": "Column added successfully"
    }))
}

async fn delete_column() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "message": "Column deleted successfully"
    }))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/tables", get(list_tables))
        .route("/api/tables/{table_id}/add_column", post(add_column))
        .route("/api/tables/{table_id}/columns/{column_name}", delete(delete_column))
}
