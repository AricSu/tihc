use crate::domain::database::Database;
use crate::domain::error::SqlEditorError;
use crate::domain::sql::{SqlMessage, SqlResult};
use base64::Engine;
use sqlx::MySqlPool;
use sqlx::{Column, Row};
use std::sync::Arc;
use time;

// --- SQL Constants ---
const MYSQL_SELECT_ONE: &str = "SELECT SCHEMA_NAME ,DEFAULT_COLLATION_NAME, DEFAULT_CHARACTER_SET_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = ?";
const MYSQL_SELECT_ALL: &str = "SELECT SCHEMA_NAME ,DEFAULT_COLLATION_NAME, DEFAULT_CHARACTER_SET_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME='tihc'";
const MYSQL_INSERT: &str = "INSERT INTO databases (name, description, created_at) VALUES (?, ?, ?)";
const MYSQL_UPDATE: &str = "UPDATE databases SET description = ?, created_at = ? WHERE name = ?";
const MYSQL_DELETE: &str = "DELETE FROM databases WHERE name = ?";

// --- 多数据库类型 trait ---
#[async_trait::async_trait]
pub trait DatabaseBackend: Send + Sync {
    async fn add(&self, db: &Database) -> Result<(), SqlEditorError>;
    async fn list(
        &self,
        pool: crate::domain::database::DatabasePool,
    ) -> Result<Vec<Database>, SqlEditorError>;
    async fn get(&self, db_name: &str) -> Result<Option<Database>, SqlEditorError>;
    async fn update(&self, db_name: &str, db: &Database) -> Result<bool, SqlEditorError>;
    async fn delete(&self, db_name: &str) -> Result<bool, SqlEditorError>;
    async fn execute_sql(&self, sql: &str) -> Result<SqlResult, SqlEditorError>;
}

pub struct DummyBackend;

#[async_trait::async_trait]
impl DatabaseBackend for DummyBackend {
    async fn add(&self, _db: &Database) -> Result<(), SqlEditorError> {
        Err(SqlEditorError::Database(
            "DummyBackend not implemented".to_string(),
        ))
    }
    async fn list(
        &self,
        _pool: crate::domain::database::DatabasePool,
    ) -> Result<Vec<Database>, SqlEditorError> {
        Err(SqlEditorError::Database(
            "DummyBackend not implemented".to_string(),
        ))
    }
    async fn get(&self, _db_name: &str) -> Result<Option<Database>, SqlEditorError> {
        Err(SqlEditorError::Database(
            "DummyBackend not implemented".to_string(),
        ))
    }
    async fn update(&self, _db_name: &str, _db: &Database) -> Result<bool, SqlEditorError> {
        Err(SqlEditorError::Database(
            "DummyBackend not implemented".to_string(),
        ))
    }
    async fn delete(&self, _db_name: &str) -> Result<bool, SqlEditorError> {
        Err(SqlEditorError::Database(
            "DummyBackend not implemented".to_string(),
        ))
    }
    async fn execute_sql(&self, _sql: &str) -> Result<SqlResult, SqlEditorError> {
        Err(SqlEditorError::Database(
            "DummyBackend not implemented".to_string(),
        ))
    }
}

// --- MySQL 实现 ---
pub struct MySqlBackend {
    pub pool: Arc<MySqlPool>,
}

#[async_trait::async_trait]
impl DatabaseBackend for MySqlBackend {
    async fn add(&self, db: &Database) -> Result<(), SqlEditorError> {
        sqlx::query(MYSQL_INSERT)
            .bind(&db.schema_name)
            .bind(&db.default_collation_name)
            .bind(&db.default_character_set_name)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| SqlEditorError::Database(e.to_string()))?;
        Ok(())
    }

    async fn list(
        &self,
        pool: crate::domain::database::DatabasePool,
    ) -> Result<Vec<Database>, SqlEditorError> {
        // 只处理 MySql 类型，后续可扩展
        match pool {
            crate::domain::database::DatabasePool::MySql(mysql_pool) => {
                let rows = sqlx::query_as::<_, Database>(MYSQL_SELECT_ALL)
                    .fetch_all(mysql_pool.as_ref())
                    .await
                    .map_err(|e| SqlEditorError::Database(e.to_string()))?;
                Ok(rows)
            }
            _ => Err(SqlEditorError::Database(
                "Unsupported pool type for MySqlBackend".to_string(),
            )),
        }
    }

    async fn get(&self, db_name: &str) -> Result<Option<Database>, SqlEditorError> {
        let row = sqlx::query_as::<_, Database>(MYSQL_SELECT_ONE)
            .bind(db_name)
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(|e| SqlEditorError::Database(e.to_string()))?;
        Ok(row)
    }

    async fn update(&self, db_name: &str, db: &Database) -> Result<bool, SqlEditorError> {
        let result = sqlx::query(MYSQL_UPDATE)
            .bind(&db.default_collation_name)
            .bind(&db.default_character_set_name)
            .bind(db_name)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| SqlEditorError::Database(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    async fn delete(&self, db_name: &str) -> Result<bool, SqlEditorError> {
        let result = sqlx::query(MYSQL_DELETE)
            .bind(db_name)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| SqlEditorError::Database(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    async fn execute_sql(&self, sql: &str) -> Result<SqlResult, SqlEditorError> {
        use std::time::Instant;
        let sql_preview = if sql.len() > 200 {
            format!("{}...", &sql[..200])
        } else {
            sql.to_string()
        };
        tracing::debug!(target: "sql_editor_backend", "Starting SQL execution: {}", sql_preview);
        let mut result = SqlResult::default();
        let start_time = Instant::now();
        let mut stream = sqlx::query(sql).fetch(self.pool.as_ref());
        let mut row_count = 0;

        while let Some(row) = futures_util::StreamExt::next(&mut stream)
            .await
            .transpose()
            .map_err(|e| {
                tracing::error!(target: "sql_editor_backend", "SQL execution failed: {} | SQL: {}", e, sql_preview);
                SqlEditorError::Database(e.to_string())
            })?
        {
            if result.column_names.is_empty() {
                result.column_names = row.columns().iter().map(|c| c.name().to_string()).collect();
                result.column_type_names = row
                    .columns()
                    .iter()
                    .map(|c| c.type_info().to_string())
                    .collect();
                tracing::debug!(target: "sql_editor_backend", "execute_sql: columns = {:?}", result.column_names);
            }
            let mut row_vec = Vec::new();
            for idx in 0..row.len() {
                let col_name = row.columns().get(idx).map(|c| c.name()).unwrap_or("");
                let col_type = row
                    .columns()
                    .get(idx)
                    .map(|c| c.type_info().to_string())
                    .unwrap_or_default();
                let (value, _) = parse_sqlx_value(&row, idx);
                tracing::trace!(target: "sql_editor_backend", "execute_sql: processing column {} ({})", col_name, col_type);
                row_vec.push(value);
            }
            tracing::trace!(target: "sql_editor_backend", "execute_sql: processed row {}", row_count);
            result.rows.push(row_vec);
            row_count += 1;
            
            // Warn about large result sets
            if row_count % 10000 == 0 {
                tracing::warn!(target: "sql_editor_backend", "Large result set detected: {} rows processed", row_count);
            }
        }
        let elapsed = start_time.elapsed();
        result.latency_ms = Some(elapsed.as_millis() as u64);
        result.rows_count = Some(row_count as u64);
        result.statement = Some(sql.to_string());
        
        // Performance monitoring
        let elapsed_ms = elapsed.as_millis();
        if elapsed_ms > 5000 {
            tracing::warn!(target: "sql_editor_backend", 
                "Slow query detected: {}ms | rows={} | sql={}", 
                elapsed_ms, row_count, sql_preview
            );
        } else if elapsed_ms > 1000 {
            tracing::info!(target: "sql_editor_backend", 
                "Query completed: rows={}, columns={}, time={}ms | sql={}", 
                row_count, 
                result.column_names.len(),
                elapsed_ms,
                sql_preview
            );
        } else {
            tracing::info!(target: "sql_editor_backend", 
                "Query completed: rows={}, columns={}, time={}ms", 
                row_count, 
                result.column_names.len(),
                elapsed_ms
            );
        }
        
        // Large result set warning
        if row_count > 50000 {
            tracing::warn!(target: "sql_editor_backend", 
                "Very large result set returned: {} rows, {} columns. Consider using LIMIT clause.", 
                row_count, result.column_names.len()
            );
        }

        let warn_rows = sqlx::query("SHOW WARNINGS")
            .fetch_all(self.pool.as_ref())
            .await
            .unwrap_or_default();
        for warn in warn_rows {
            let level: String = warn.try_get("Level").unwrap_or_default();
            let content: String = warn.try_get("Message").unwrap_or_default();
            if !level.is_empty() || !content.is_empty() {
                if let Some(messages) = result.messages.as_mut() {
                    messages.push(SqlMessage { level, content });
                }
            }
        }
        Ok(result)
    }
}

/// 统一解析 sqlx::Row 的字段，返回 (值, debug)
fn parse_sqlx_value(row: &sqlx::mysql::MySqlRow, idx: usize) -> (serde_json::Value, String) {
    use serde_json::Value;
    // 优先类型解码
    macro_rules! try_type {
        ($ty:ty, $desc:expr, $fmt:expr) => {
            if let Ok(Some(v)) = row.try_get::<Option<$ty>, _>(idx) {
                return ($fmt(v.clone()), format!(concat!($desc, ": {:?}"), &v));
            }
        };
    }
    try_type!(
        chrono::NaiveDateTime,
        "chrono::NaiveDateTime",
        |v: chrono::NaiveDateTime| Value::String(v.format("%Y-%m-%d %H:%M:%S%.6f").to_string())
    );
    try_type!(
        chrono::NaiveDate,
        "chrono::NaiveDate",
        |v: chrono::NaiveDate| Value::String(v.format("%Y-%m-%d").to_string())
    );
    try_type!(
        chrono::NaiveTime,
        "chrono::NaiveTime",
        |v: chrono::NaiveTime| Value::String(v.format("%H:%M:%S").to_string())
    );
    try_type!(
        time::PrimitiveDateTime,
        "time::PrimitiveDateTime",
        |v: time::PrimitiveDateTime| Value::String(v.to_string())
    );
    try_type!(time::Date, "time::Date", |v: time::Date| Value::String(
        v.to_string()
    ));
    try_type!(time::Time, "time::Time", |v: time::Time| Value::String(
        v.to_string()
    ));
    try_type!(String, "String", |v: String| Value::String(v.clone()));
    try_type!(i64, "i64", |v: i64| Value::String(v.to_string()));
    try_type!(u64, "u64", |v: u64| Value::String(v.to_string()));
    try_type!(f64, "f64", |v: f64| Value::String(v.to_string()));
    try_type!(bool, "bool", |v: bool| Value::String(v.to_string()));
    try_type!(Vec<u8>, "Vec<u8>", |v: Vec<u8>| {
        if v.is_empty() {
            Value::String(String::new())
        } else {
            Value::String(base64::engine::general_purpose::STANDARD.encode(&v))
        }
    });
    try_type!(
        serde_json::Value,
        "serde_json::Value",
        |v: serde_json::Value| v.clone()
    );

    // 兜底 decode_raw
    match row.try_get_raw(idx) {
        Ok(v) => {
            if let Ok(dt) =
                <chrono::NaiveDateTime as sqlx::decode::Decode<sqlx::MySql>>::decode(v.clone())
            {
                return (
                    Value::String(dt.format("%Y-%m-%d %H:%M:%S%.6f").to_string()),
                    format!("decode_raw chrono::NaiveDateTime: {:?}", dt),
                );
            } else if let Ok(dt) =
                <time::PrimitiveDateTime as sqlx::decode::Decode<sqlx::MySql>>::decode(v.clone())
            {
                return (
                    Value::String(dt.to_string()),
                    format!("decode_raw time::PrimitiveDateTime: {:?}", dt),
                );
            } else if let Ok(s) = <String as sqlx::decode::Decode<sqlx::MySql>>::decode(v.clone()) {
                return (
                    Value::String(s.clone()),
                    format!("decode_raw String: {:?}", s),
                );
            } else if let Ok(b) = <Vec<u8> as sqlx::decode::Decode<sqlx::MySql>>::decode(v.clone())
            {
                return (
                    Value::String(base64::engine::general_purpose::STANDARD.encode(&b)),
                    format!("decode_raw Vec<u8>: {:?}", b),
                );
            } else {
                return (Value::Null, "decode_raw NULL".to_string());
            }
        }
        Err(e) => (Value::Null, format!("get_raw error: {}", e)),
    }
    // END parse_sqlx_value
}

// --- DatabaseStore 动态分发 ---
pub struct DatabaseStore {
    pub backend: Arc<dyn DatabaseBackend>,
    pub connection_store: Arc<crate::infrastructure::connection_store::ConnectionStore>,
}

impl DatabaseStore {
    pub fn new_dummy(
        connection_store: Arc<crate::infrastructure::connection_store::ConnectionStore>,
    ) -> Self {
        Self {
            backend: Arc::new(DummyBackend),
            connection_store,
        }
    }
    pub fn new_mysql(
        pool: Arc<MySqlPool>,
        connection_store: Arc<crate::infrastructure::connection_store::ConnectionStore>,
    ) -> Self {
        Self {
            backend: Arc::new(MySqlBackend { pool }),
            connection_store,
        }
    }
    // 未来可扩展 new_postgres/new_sqlite 等
}

// --- 用法示例 ---
// let store = DatabaseStore::new_mysql(mysql_pool);
// store.backend.add(&db).await?;
