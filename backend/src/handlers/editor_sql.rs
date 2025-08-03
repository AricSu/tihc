use axum::{Extension, Json};
use std::sync::Arc;
use microkernel::platform::{CommandRegistry, ServiceRegistry};
use crate::api::editor_sql::{ExecuteSqlRequest, SqlResult};

/// HTTP API handler: /api/sql/execute
pub async fn handle_execute_sql(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(req): Json<ExecuteSqlRequest>,
) -> Json<SqlResult> {
    tracing::info!(target: "editor_sql_handler", "handle_execute_sql called, connection_id={}, sql={}", req.connection_id, req.sql);
    execute_sql(Extension(registry), Json(req)).await
}


/// 执行 SQL 查询，仅支持 SELECT
/// POST /api/sql/execute
pub async fn execute_sql(
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
    match result {
        Ok(val) => {
            if let Err(e) = serde_json::from_value::<SqlResult>(val.clone()) {
                tracing::error!(target: "editor_sql_handler", "SqlResult parse error: {}. Raw value: {:?}", e, val);
            }
            Json(serde_json::from_value(val.clone()).unwrap_or_else(|_| SqlResult {
                column_names: vec![],
                column_type_names: vec![],
                rows: vec![],
                rows_count: Some(0),
                error: Some("结果解析失败".to_string()),
                latency_ms: None,
                statement: Some(sql.to_string()),
                messages: None,
            }))
        }
        Err(e) => {
            tracing::error!(target: "editor_sql_handler", "SQL execution error: {}", e);
            Json(SqlResult {
                column_names: vec![],
                column_type_names: vec![],
                rows: vec![],
                rows_count: Some(0),
                error: Some(e.to_string()),
                latency_ms: None,
                statement: Some(sql.to_string()),
                messages: None,
            })
        }
    }
}
