use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{get, post},
};
use microkernel::platform::ServiceRegistry;
use microkernel::platform::command_registry::CommandRegistry;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ExecuteSqlRequest {
    pub connection_id: u64,
    pub sql: String,
}

#[derive(Serialize)]
pub struct SqlResult {
    pub column_names: Vec<String>,
    pub column_type_names: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub rows_count: Option<u64>,
    pub error: Option<String>,
    pub latency_ms: Option<u64>,
    pub statement: Option<String>,
    pub messages: Option<Vec<SqlMessage>>,
}

#[derive(Serialize)]
pub struct SqlMessage {
    pub level: String,
    pub content: String,
}

async fn execute_sql(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(req): Json<ExecuteSqlRequest>,
) -> Json<SqlResult> {
    let sql = req.sql.trim();
    let connection_id = req.connection_id;
    if !sql.to_lowercase().starts_with("select") {
        return Json(SqlResult {
            column_names: vec![],
            column_type_names: vec![],
            rows: vec![],
            rows_count: Some(0),
            error: Some("只允许 SELECT 查询".to_string()),
            latency_ms: None,
            statement: Some(sql.to_string()),
            messages: None,
        });
    }
    let start = std::time::Instant::now();
    let result = if let Some(cmd_reg) = registry.resolve::<Box<CommandRegistry>>() {
        cmd_reg
            .execute(
                "editor-sql-execute",
                &[connection_id.to_string(), sql.to_string()],
            )
            .await
    } else {
        Err(anyhow::anyhow!("command registry not found"))
    };
    let latency = start.elapsed().as_millis() as u64;
    match result {
        Ok(val) => {
            let column_names = val
                .get("columns")
                .and_then(|c| c.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            let column_type_names = val
                .get("column_types")
                .and_then(|c| c.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            let rows: Vec<Vec<serde_json::Value>> = val
                .get("rows")
                .and_then(|r| r.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|row| row.as_array().map(|v| v.to_vec()))
                        .collect()
                })
                .unwrap_or_default();
            let rows_count = Some(rows.len() as u64);
            let messages = val.get("messages").and_then(|m| m.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|msg| {
                        let level = msg
                            .get("level")
                            .and_then(|l| l.as_str())
                            .unwrap_or("")
                            .to_string();
                        let content = msg
                            .get("content")
                            .and_then(|c| c.as_str())
                            .unwrap_or("")
                            .to_string();
                        if !level.is_empty() || !content.is_empty() {
                            Some(SqlMessage { level, content })
                        } else {
                            None
                        }
                    })
                    .collect()
            });
            Json(SqlResult {
                column_names,
                column_type_names,
                rows,
                rows_count,
                error: None,
                latency_ms: Some(latency),
                statement: Some(sql.to_string()),
                messages,
            })
        }
        Err(e) => Json(SqlResult {
            column_names: vec![],
            column_type_names: vec![],
            rows: vec![],
            rows_count: Some(0),
            error: Some(e.to_string()),
            latency_ms: Some(latency),
            statement: Some(sql.to_string()),
            messages: None,
        }),
    }
}

async fn sql_status(Path(_task_id): Path<u64>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "completed",
        "message": "SQL query executed successfully",
        "data": []
    }))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/sql/execute", post(execute_sql))
        .route("/api/sql/status/{task_id}", get(sql_status))
}
