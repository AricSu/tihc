use crate::domain::database::Database;
use crate::domain::error::SqlEditorError;
use crate::domain::sql::{SqlMessage, SqlQueryResult};
use sqlx::MySqlPool;
use std::sync::Arc;

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
    async fn list(&self, pool: crate::domain::database::DatabasePool) -> Result<Vec<Database>, SqlEditorError>;
    async fn get(&self, db_name: &str) -> Result<Option<Database>, SqlEditorError>;
    async fn update(&self, db_name: &str, db: &Database) -> Result<bool, SqlEditorError>;
    async fn delete(&self, db_name: &str) -> Result<bool, SqlEditorError>;
    async fn execute_sql(&self, sql: &str) -> Result<SqlQueryResult, SqlEditorError>;
}

pub struct DummyBackend;

#[async_trait::async_trait]
impl DatabaseBackend for DummyBackend {
    async fn add(&self, _db: &Database) -> Result<(), SqlEditorError> {
        Err(SqlEditorError::Database(
            "DummyBackend not implemented".to_string(),
        ))
    }
    async fn list(&self, _pool: crate::domain::database::DatabasePool) -> Result<Vec<Database>, SqlEditorError> {
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
    async fn execute_sql(&self, _sql: &str) -> Result<SqlQueryResult, SqlEditorError> {
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

    async fn list(&self, pool: crate::domain::database::DatabasePool) -> Result<Vec<Database>, SqlEditorError> {
        // 只处理 MySql 类型，后续可扩展
        match pool {
            crate::domain::database::DatabasePool::MySql(mysql_pool) => {
                let rows = sqlx::query_as::<_, Database>(MYSQL_SELECT_ALL)
                    .fetch_all(mysql_pool.as_ref())
                    .await
                    .map_err(|e| SqlEditorError::Database(e.to_string()))?;
                Ok(rows)
            }
            _ => Err(SqlEditorError::Database("Unsupported pool type for MySqlBackend".to_string())),
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

    async fn execute_sql(&self, sql: &str) -> Result<SqlQueryResult, SqlEditorError> {
        use sqlx::{Column, Row};
        let mut result = SqlQueryResult::default();
        let mut stream = sqlx::query(sql).fetch(self.pool.as_ref());
        while let Some(row) = futures_util::StreamExt::next(&mut stream)
            .await
            .transpose()
            .map_err(|e| SqlEditorError::Database(e.to_string()))?
        {
            if result.columns.is_empty() {
                result.columns = row.columns().iter().map(|c| c.name().to_string()).collect();
                result.column_types = row
                    .columns()
                    .iter()
                    .map(|c| c.type_info().to_string())
                    .collect();
            }
            let mut row_vec = Vec::new();
            for idx in 0..row.len() {
                let v: serde_json::Value = row.try_get(idx).unwrap_or(serde_json::Value::Null);
                row_vec.push(v);
            }
            result.rows.push(row_vec);
        }
        let warn_rows = sqlx::query("SHOW WARNINGS")
            .fetch_all(self.pool.as_ref())
            .await
            .unwrap_or_default();
        for warn in warn_rows {
            let level: String = warn.try_get("Level").unwrap_or_default();
            let content: String = warn.try_get("Message").unwrap_or_default();
            if !level.is_empty() || !content.is_empty() {
                result.messages.push(SqlMessage { level, content });
            }
        }
        Ok(result)
    }
}

// --- DatabaseStore 动态分发 ---
pub struct DatabaseStore {
    pub backend: Arc<dyn DatabaseBackend>,
    pub connection_store: Arc<crate::infrastructure::connection_store::ConnectionStore>,
}

impl DatabaseStore {
    pub fn new_dummy(connection_store: Arc<crate::infrastructure::connection_store::ConnectionStore>) -> Self {
        Self {
            backend: Arc::new(DummyBackend),
            connection_store,
        }
    }
    pub fn new_mysql(pool: Arc<MySqlPool>, connection_store: Arc<crate::infrastructure::connection_store::ConnectionStore>) -> Self {
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
