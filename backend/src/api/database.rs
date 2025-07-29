use axum::{extract::{Path, Json}, Router, routing::{get, post, delete}, Extension};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use core::platform::ServiceRegistry;
use core::platform::command_registry::CommandRegistry;



#[derive(Deserialize, Serialize, Debug)]
pub struct DBConnectionnRequest {
    pub id: u64,
    pub name: String,
    pub engine: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub created_at: String,
}

fn exec_cmd(registry: &Arc<ServiceRegistry>, cmd: &str, args: Vec<String>) -> serde_json::Value {
    if let Some(cmd_reg) = registry.resolve::<Box<CommandRegistry>>() {
        match cmd_reg.execute(cmd, &args) {
            Ok(result) => json!(result),
            Err(e) => json!({"error": e.to_string()}),
        }
    } else {
        json!({"error": "command registry not found"})
    }
}

pub async fn update_connection(Extension(registry): Extension<Arc<ServiceRegistry>>, Path(conn_id): Path<u64>, Json(req): Json<DBConnectionnRequest>) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-connections-update", vec![conn_id.to_string(), serde_json::to_string(&req).unwrap()]))
}

pub async fn get_connection(Extension(registry): Extension<Arc<ServiceRegistry>>, Path(conn_id): Path<u64>) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-connections-get", vec![conn_id.to_string()]))
}

pub async fn list_connections(Extension(registry): Extension<Arc<ServiceRegistry>>) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-connections-list", vec![]))
}

pub async fn create_connection(Extension(registry): Extension<Arc<ServiceRegistry>>, Json(req): Json<DBConnectionnRequest>) -> Json<serde_json::Value> {
    tracing::info!(target: "backend_api", "[create_connection] called, req={:?}", req);
    Json(exec_cmd(&registry, "editor-connections-create", vec![serde_json::to_string(&req).unwrap()]))
}

pub async fn delete_connection(Extension(registry): Extension<Arc<ServiceRegistry>>, Path(conn_id): Path<u64>) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-connections-delete", vec![conn_id.to_string()]))
}
// 表相关 API
pub async fn list_tables(Extension(registry): Extension<Arc<ServiceRegistry>>) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-tables-list", vec![]))
}

pub async fn add_table(Extension(registry): Extension<Arc<ServiceRegistry>>, Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-tables-add", vec![req.to_string()]))
}

pub async fn delete_table(Extension(registry): Extension<Arc<ServiceRegistry>>, Path(table_name): Path<String>) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-tables-delete", vec![table_name]))
}

pub async fn add_column(Extension(registry): Extension<Arc<ServiceRegistry>>, Path(table_name): Path<String>, Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    // req: column json
    Json(exec_cmd(&registry, "editor-tables-add-column", vec![table_name, req.to_string()]))
}

pub async fn delete_column(Extension(registry): Extension<Arc<ServiceRegistry>>, Path((table_name, column_name)): Path<(String, String)>) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-tables-delete-column", vec![table_name, column_name]))
}

pub async fn test_connection(Json(req): Json<DBConnectionnRequest>) -> Json<serde_json::Value> {
    let success = req.host != "fail";
    if success {
        Json(json!({ "status": "success", "message": "Connection successful" }))
    } else {
        Json(json!({ "status": "error", "message": "Connection failed" }))
    }
}

// 数据库管理 API
pub async fn list_databases() -> Json<serde_json::Value> {
    // TODO: Replace with real DB query
    Json(json!({
        "data": [
            {"id": 1, "name": "database1", "engine": "MySQL", "createdAt": "2023-01-01T00:00:00Z"},
            {"id": 2, "name": "database2", "engine": "PostgreSQL", "createdAt": "2023-02-01T00:00:00Z"}
        ]
    }))
}

#[derive(Deserialize)]
pub struct CreateDatabaseRequest {
    pub name: String,
    pub engine: String,
}

pub async fn create_database(Json(req): Json<CreateDatabaseRequest>) -> Json<serde_json::Value> {
    // TODO: Replace with real DB create logic
    Json(json!({
        "id": 3,
        "name": req.name,
        "engine": req.engine,
        "createdAt": "2023-07-01T00:00:00Z"
    }))
}

pub async fn delete_database(Path(database_id): Path<u64>) -> Json<serde_json::Value> {
    let _ = database_id; // For now, just suppress unused warning
    // TODO: Replace with real DB delete logic
    Json(json!({
        "status": "success",
        "message": "Database deleted successfully"
    }))
}

pub fn routes(registry: Arc<ServiceRegistry>) -> Router {
    Router::new()
        // 连接管理
        .route("/api/connections/create", post(create_connection))
        .route("/api/connections/list", get(list_connections))
        .route("/api/connections/{id}", delete(delete_connection))
        .route("/api/connections/{id}", get(get_connection).put(update_connection))
        .route("/api/connections/test", post(test_connection))
        // 数据库管理
        .route("/api/databases", get(list_databases).post(create_database))
        .route("/api/databases/{database_id}", delete(delete_database))
        .layer(Extension(registry))
}
