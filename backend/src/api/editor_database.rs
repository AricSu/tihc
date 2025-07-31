use axum::{
    Extension, Router,
    extract::{Json, Path},
    routing::{delete, get, post},
};
use core::platform::ServiceRegistry;
use core::platform::command_registry::CommandRegistry;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
pub struct DBConnectionRequest {
    pub id: u64,
    pub name: String,
    pub engine: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub use_tls: bool,
    pub ca_cert_path: Option<String>,
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

pub async fn update_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(conn_id): Path<u64>,
    Json(req): Json<DBConnectionRequest>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(
        &registry,
        "editor-connections-update",
        vec![conn_id.to_string(), serde_json::to_string(&req).unwrap()],
    ))
}

pub async fn get_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(conn_id): Path<u64>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(
        &registry,
        "editor-connections-get",
        vec![conn_id.to_string()],
    ))
}

pub async fn list_connections(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-connections-list", vec![]))
}

pub async fn create_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(req): Json<DBConnectionRequest>,
) -> Json<serde_json::Value> {
    tracing::info!(target: "backend_api", "[create_connection] called, engine={}, host={}, port={}, user={}, db={}, use_tls={}, ca_cert_path={:?}", req.engine, req.host, req.port, req.username, req.database.as_deref().unwrap_or(""), req.use_tls, req.ca_cert_path);
    Json(exec_cmd(
        &registry,
        "editor-connections-create",
        vec![serde_json::to_string(&req).unwrap()],
    ))
}

pub async fn delete_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(conn_id): Path<u64>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(
        &registry,
        "editor-connections-delete",
        vec![conn_id.to_string()],
    ))
}
// 表相关 API
pub async fn list_tables(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-tables-list", vec![]))
}

pub async fn add_table(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(
        &registry,
        "editor-tables-add",
        vec![req.to_string()],
    ))
}

pub async fn delete_table(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(table_name): Path<String>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(
        &registry,
        "editor-tables-delete",
        vec![table_name],
    ))
}

pub async fn add_column(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(table_name): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // req: column json
    Json(exec_cmd(
        &registry,
        "editor-tables-add-column",
        vec![table_name, req.to_string()],
    ))
}

pub async fn delete_column(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path((table_name, column_name)): Path<(String, String)>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(
        &registry,
        "editor-tables-delete-column",
        vec![table_name, column_name],
    ))
}

pub async fn test_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(req): Json<DBConnectionRequest>,
) -> Json<serde_json::Value> {
    let args = vec![serde_json::to_string(&req).unwrap()];
    Json(exec_cmd(&registry, "editor-connections-test", args))
}

// 数据库管理 API
pub async fn list_databases() -> Json<serde_json::Value> {
    // 返回静态 schema 示例（可替换为真实 DB 查询）
    Json(json!({
        "data": [
            {
                "id": 1,
                "name": "cluster_slow_query",
                "engine": "TiDB",
                "createdAt": "2023-01-01T00:00:00Z",
                "schema": [
                    {"column_name": "Time", "data_type": "timestamp", "comment": "SQL 执行时间"},
                    {"column_name": "Query", "data_type": "text", "comment": "SQL 语句"},
                    {"column_name": "User", "data_type": "varchar", "comment": "执行用户"},
                    {"column_name": "Host", "data_type": "varchar", "comment": "客户端地址"},
                    {"column_name": "DB", "data_type": "varchar", "comment": "数据库名"},
                    {"column_name": "Query_time", "data_type": "float", "comment": "执行耗时 (秒)"},
                    {"column_name": "Process_time", "data_type": "float", "comment": "处理耗时 (秒)"},
                    {"column_name": "Wait_time", "data_type": "float", "comment": "等待耗时 (秒)"},
                    {"column_name": "Lock_time", "data_type": "float", "comment": "锁等待耗时 (秒)"},
                    {"column_name": "Rows_sent", "data_type": "int", "comment": "返回行数"},
                    {"column_name": "Rows_examined", "data_type": "int", "comment": "扫描行数"}
                ]
            }
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
        .route(
            "/api/connections/{id}",
            get(get_connection).put(update_connection),
        )
        .route("/api/connections/test", post(test_connection))
        // 数据库管理
        .route("/api/databases/list", get(list_databases).post(create_database))
        .route("/api/databases/{database_id}", delete(delete_database))
        .layer(Extension(registry))
}
