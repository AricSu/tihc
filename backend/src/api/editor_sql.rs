use axum::{ Json, Router,
    extract::Path,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use crate::handlers::editor_sql::handle_execute_sql;

/// SQL 执行请求体
#[derive(Deserialize)]
pub struct ExecuteSqlRequest {
    /// 连接 ID
    pub connection_id: u64,
    /// 待执行 SQL
    pub sql: String,
}


/// SQL 执行结果
#[derive(Serialize, Deserialize)]
pub struct SqlResult {
    /// 列名
    pub column_names: Vec<String>,
    /// 列类型名
    pub column_type_names: Vec<String>,
    /// 行数据
    pub rows: Vec<Vec<serde_json::Value>>,
    /// 行数
    pub rows_count: Option<u64>,
    /// 错误信息
    pub error: Option<String>,
    /// 执行耗时（毫秒）
    pub latency_ms: Option<u64>,
    /// SQL 语句
    pub statement: Option<String>,
    /// 附加消息
    pub messages: Option<Vec<SqlMessage>>,
}


/// SQL 执行消息
#[derive(Serialize, Deserialize)]
pub struct SqlMessage {
    /// 消息级别（info/warn/error）
    pub level: String,
    /// 消息内容
    pub content: String,
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
        .route("/api/sql_editor/sql/execute", post(handle_execute_sql))
        .route("/api/sql_editor/status/{task_id}", get(sql_status))
}
