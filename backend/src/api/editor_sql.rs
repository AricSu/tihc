use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ExecuteSqlRequest {
    pub database_id: u64,
    pub sql: String,
}

#[derive(Serialize)]
pub struct SqlResult {
    pub status: String,
    pub data: Vec<serde_json::Value>,
}

async fn execute_sql(Json(_req): Json<ExecuteSqlRequest>) -> Json<SqlResult> {
    Json(SqlResult {
        status: "success".to_string(),
        data: vec![],
    })
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
