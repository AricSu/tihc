use crate::domain::database::DatabaseConnection;
use crate::domain::error::SqlEditorError;
use serde_json::Value;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Column;
use sqlx::Row;

/// SQL 执行器，负责实际 DDL/DML 执行
pub struct SqlExecutor;

impl SqlExecutor {
    /// 执行 SQL，支持 MySQL/TiDB
    pub async fn execute_sql(
        conn: &DatabaseConnection,
        sql: &str,
    ) -> Result<Value, SqlEditorError> {
        // 构造连接字符串，直接用字段访问
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            conn.username,
            conn.password.as_deref().unwrap_or(""),
            conn.host,
            conn.port,
            conn.database.as_deref().unwrap_or("")
        );
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .map_err(|e| {
                SqlEditorError::InfraCommon(crate::common::error::CommonError::Other(e.to_string()))
            })?;
        // 判断 DML/DDL/Query
        if sql.trim().to_lowercase().starts_with("select") {
            let rows = sqlx::query(sql).fetch_all(&pool).await.map_err(|e| {
                SqlEditorError::InfraCommon(crate::common::error::CommonError::Other(e.to_string()))
            })?;
            let result: Vec<_> = rows
                .iter()
                .map(|row| {
                    let mut obj = serde_json::Map::new();
                    for (i, col) in row.columns().iter().enumerate() {
                        let val = row
                            .try_get::<String, _>(i)
                            .map(Value::String)
                            .unwrap_or(Value::Null);
                        obj.insert(col.name().to_string(), val);
                    }
                    Value::Object(obj)
                })
                .collect();
            Ok(Value::Array(result))
        } else {
            let res = sqlx::query(sql).execute(&pool).await.map_err(|e| {
                SqlEditorError::InfraCommon(crate::common::error::CommonError::Other(e.to_string()))
            })?;
            Ok(serde_json::json!({ "rows_affected": res.rows_affected() }))
        }
    }
}
