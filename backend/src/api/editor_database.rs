use axum::{
    Extension, Router,
    extract::{Json, Path},
    routing::{delete, get, post},
};
use common::json_resp;
use microkernel::platform::ServiceRegistry;
use microkernel::platform::command_registry::CommandRegistry;
use serde::{Deserialize, Serialize};
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

async fn exec_cmd(
    registry: &Arc<ServiceRegistry>,
    cmd: &str,
    args: Vec<String>,
) -> serde_json::Value {
    if let Some(cmd_reg) = registry.resolve::<Box<CommandRegistry>>() {
        match cmd_reg.execute(cmd, &args).await {
            Ok(result) => serde_json::to_value(json_resp!(success, result)).unwrap(),
            Err(e) => serde_json::to_value(json_resp!(Failed, e.to_string())).unwrap(),
        }
    } else {
        serde_json::to_value(json_resp!(NotFound, Some("command registry not found"))).unwrap()
    }
}

pub async fn update_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(conn_id): Path<u64>,
    Json(req): Json<DBConnectionRequest>,
) -> Json<serde_json::Value> {
    Json(
        exec_cmd(
            &registry,
            "editor-connections-update",
            vec![conn_id.to_string(), serde_json::to_string(&req).unwrap()],
        )
        .await,
    )
}

pub async fn get_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(conn_id): Path<u64>,
) -> Json<serde_json::Value> {
    Json(
        exec_cmd(
            &registry,
            "editor-connections-get",
            vec![conn_id.to_string()],
        )
        .await,
    )
}

pub async fn list_connections(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-connections-list", vec![]).await)
}

pub async fn create_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(req): Json<DBConnectionRequest>,
) -> Json<serde_json::Value> {
    tracing::info!(target: "backend_api", "[create_connection] called, engine={}, host={}, port={}, user={}, db={}, use_tls={}, ca_cert_path={:?}", req.engine, req.host, req.port, req.username, req.database.as_deref().unwrap_or(""), req.use_tls, req.ca_cert_path);
    Json(
        exec_cmd(
            &registry,
            "editor-connections-create",
            vec![serde_json::to_string(&req).unwrap()],
        )
        .await,
    )
}

pub async fn delete_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(conn_id): Path<u64>,
) -> Json<serde_json::Value> {
    Json(
        exec_cmd(
            &registry,
            "editor-connections-delete",
            vec![conn_id.to_string()],
        )
        .await,
    )
}

// 表相关 API
pub async fn list_tables(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let database = params.get("database").cloned().unwrap_or_default();
    let connection_id = params.get("connection_id").cloned().unwrap_or_default();
    let mut args = vec![];
    if !connection_id.is_empty() {
        args.push(connection_id);
    }
    if !database.is_empty() {
        args.push(database);
    }
    Json(exec_cmd(&registry, "editor-tables-list", args).await)
}

pub async fn add_table(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-tables-add", vec![req.to_string()]).await)
}

pub async fn delete_table(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(table_name): Path<String>,
) -> Json<serde_json::Value> {
    Json(exec_cmd(&registry, "editor-tables-delete", vec![table_name]).await)
}

pub async fn list_columns(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let schema = params.get("schema").cloned().unwrap_or_default();
    let table = params.get("table").cloned().unwrap_or_default();
    let connection_id = params.get("connection_id").cloned().unwrap_or_default();
    let mut args = vec![];
    if !connection_id.is_empty() {
        args.push(connection_id);
    }
    if !schema.is_empty() {
        args.push(schema);
    }
    if !table.is_empty() {
        args.push(table);
    }
    Json(exec_cmd(&registry, "editor-columns-list", args).await)
}

// 索引相关 API
pub async fn list_indexes(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let schema = params.get("schema").cloned().unwrap_or_default();
    let table = params.get("table").cloned().unwrap_or_default();
    let connection_id = params.get("connection_id").cloned().unwrap_or_default();
    let mut args = vec![];
    if !connection_id.is_empty() {
        args.push(connection_id);
    }
    if !schema.is_empty() {
        args.push(schema);
    }
    if !table.is_empty() {
        args.push(table);
    }
    Json(exec_cmd(&registry, "editor-indexes-list", args).await)
}

pub async fn add_column(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path(table_name): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // req: column json
    Json(
        exec_cmd(
            &registry,
            "editor-tables-add-column",
            vec![table_name, req.to_string()],
        )
        .await,
    )
}

pub async fn delete_column(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Path((table_name, column_name)): Path<(String, String)>,
) -> Json<serde_json::Value> {
    Json(
        exec_cmd(
            &registry,
            "editor-tables-delete-column",
            vec![table_name, column_name],
        )
        .await,
    )
}

pub async fn test_connection(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(req): Json<DBConnectionRequest>,
) -> Json<serde_json::Value> {
    let args = vec![serde_json::to_string(&req).unwrap()];
    Json(exec_cmd(&registry, "editor-connections-test", args).await)
}

// 数据库管理 API
use axum::extract::Query;
use std::collections::HashMap;

pub async fn list_databases(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let connection_id = params.get("connection_id").cloned().unwrap_or_default();
    let args = if !connection_id.is_empty() {
        vec![connection_id]
    } else {
        vec![]
    };
    Json(exec_cmd(&registry, "editor-databases-list", args).await)
}

pub fn routes(registry: Arc<ServiceRegistry>) -> Router {
    Router::new()
        // 连接管理
        .route(
            "/sql_editor/connections/create",
            post(create_connection),
        )
        .route("/sql_editor/connections/list", get(list_connections))
        .route(
            "/sql_editor/connections/{id}",
            delete(delete_connection),
        )
        .route(
            "/sql_editor/connections/{id}",
            get(get_connection).put(update_connection),
        )
        .route("/sql_editor/connections/test", post(test_connection))
        // 数据库管理
        .route("/sql_editor/databases/list", get(list_databases))
        .route("/sql_editor/tables/list", get(list_tables))
        // 列与索引管理
        .route("/sql_editor/columns/list", get(list_columns))
        .route("/sql_editor/indexes/list", get(list_indexes))
        .layer(Extension(registry))
}
