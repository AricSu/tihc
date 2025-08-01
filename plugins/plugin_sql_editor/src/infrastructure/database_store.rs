use crate::domain::database::DatabaseConnection;
use crate::domain::error::SqlEditorError;
use crate::domain::sql::{SqlMessage, SqlQueryResult};

impl DatabaseStore {
    /// Executes a SQL query and collects columns, types, rows, and messages.
    pub async fn execute_sql(&self, sql: &str) -> Result<SqlQueryResult, SqlEditorError> {
        use sqlx::{Column, Row};
        let mut result = SqlQueryResult::default();
        match &self.pool {
            DbPool::MySql(mysql) => {
                let mut stream = sqlx::query(sql).fetch(mysql);
                while let Some(row) = futures_util::StreamExt::next(&mut stream)
                    .await
                    .transpose()
                    .map_err(|e| SqlEditorError::Database(e.to_string()))?
                {
                    if result.columns.is_empty() {
                        result.columns =
                            row.columns().iter().map(|c| c.name().to_string()).collect();
                        result.column_types = row
                            .columns()
                            .iter()
                            .map(|c| c.type_info().to_string())
                            .collect();
                    }
                    let mut row_vec = Vec::new();
                    for idx in 0..row.len() {
                        let v: serde_json::Value =
                            row.try_get(idx).unwrap_or(serde_json::Value::Null);
                        row_vec.push(v);
                    }
                    result.rows.push(row_vec);
                }
                let warn_rows = sqlx::query("SHOW WARNINGS")
                    .fetch_all(mysql)
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
            _ => Err(SqlEditorError::Other("unsupported pool type".to_string())),
        }
    }
    /// 根据 connection 信息动态创建 MySQL 连接池
    pub fn with_mysql(conn: &DatabaseConnection) -> Self {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            conn.username,
            conn.password.as_deref().unwrap_or_default(),
            conn.host,
            conn.port,
            conn.database.as_deref().unwrap_or_default()
        );
        let pool = MySqlPool::connect_lazy(&url).expect("Failed to create MySQL pool");
        DatabaseStore {
            pool: DbPool::MySql(pool),
        }
    }

    /// 根据 connection 信息动态创建 Postgres 连接池
    pub fn with_postgres(conn: &DatabaseConnection) -> Self {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            conn.username,
            conn.password.as_deref().unwrap_or_default(),
            conn.host,
            conn.port,
            conn.database.as_deref().unwrap_or_default()
        );
        let pool = PgPool::connect_lazy(&url).expect("Failed to create Postgres pool");
        DatabaseStore {
            pool: DbPool::Postgres(pool),
        }
    }

    /// 构造 Dummy 数据库存储
    pub fn dummy() -> Self {
        DatabaseStore {
            pool: DbPool::Dummy,
        }
    }
}
use crate::domain::database::Database;
use sqlx::{MySqlPool, PgPool};

#[derive(Clone)]
pub enum DbPool {
    MySql(MySqlPool),
    Postgres(PgPool),
    Dummy,
}

pub struct DatabaseStore {
    pub pool: DbPool,
}

impl DatabaseStore {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// 添加数据库/schema
    pub async fn add(&self, db: &Database) -> Result<(), sqlx::Error> {
        match &self.pool {
            DbPool::MySql(mysql) => {
                sqlx::query(
                    "INSERT INTO databases (name, description, created_at) VALUES (?, ?, ?)",
                )
                .bind(&db.name)
                .bind(&db.description)
                .bind(&db.created_at)
                .execute(mysql)
                .await?;
                Ok(())
            }
            DbPool::Postgres(pg) => {
                sqlx::query(
                    "INSERT INTO databases (name, description, created_at) VALUES ($1, $2, $3)",
                )
                .bind(&db.name)
                .bind(&db.description)
                .bind(&db.created_at)
                .execute(pg)
                .await?;
                Ok(())
            }
            DbPool::Dummy => Err(sqlx::Error::Protocol(
                "Dummy pool: operation not supported".into(),
            )),
        }
    }

    /// 列出所有数据库/schema
    pub async fn list(&self) -> Result<Vec<Database>, sqlx::Error> {
        match &self.pool {
            DbPool::MySql(mysql) => {
                let rows = sqlx::query_as::<_, Database>(
                    "SELECT name, description, created_at FROM databases",
                )
                .fetch_all(mysql)
                .await?;
                Ok(rows)
            }
            DbPool::Postgres(pg) => {
                let rows = sqlx::query_as::<_, Database>(
                    "SELECT name, description, created_at FROM databases",
                )
                .fetch_all(pg)
                .await?;
                Ok(rows)
            }
            DbPool::Dummy => Err(sqlx::Error::Protocol(
                "Dummy pool: operation not supported".into(),
            )),
        }
    }

    /// 按名查找数据库/schema
    pub async fn get(&self, db_name: &str) -> Result<Option<Database>, sqlx::Error> {
        match &self.pool {
            DbPool::MySql(mysql) => {
                let row = sqlx::query_as::<_, Database>(
                    "SELECT name, description, created_at FROM databases WHERE name = ?",
                )
                .bind(db_name)
                .fetch_optional(mysql)
                .await?;
                Ok(row)
            }
            DbPool::Postgres(pg) => {
                let row = sqlx::query_as::<_, Database>(
                    "SELECT name, description, created_at FROM databases WHERE name = $1",
                )
                .bind(db_name)
                .fetch_optional(pg)
                .await?;
                Ok(row)
            }
            DbPool::Dummy => Err(sqlx::Error::Protocol(
                "Dummy pool: operation not supported".into(),
            )),
        }
    }

    /// 更新数据库/schema（全量覆盖）
    pub async fn update(&self, db_name: &str, db: &Database) -> Result<bool, sqlx::Error> {
        match &self.pool {
            DbPool::MySql(mysql) => {
                let result = sqlx::query(
                    "UPDATE databases SET description = ?, created_at = ? WHERE name = ?",
                )
                .bind(&db.description)
                .bind(&db.created_at)
                .bind(db_name)
                .execute(mysql)
                .await?;
                Ok(result.rows_affected() > 0)
            }
            DbPool::Postgres(pg) => {
                let result = sqlx::query(
                    "UPDATE databases SET description = $1, created_at = $2 WHERE name = $3",
                )
                .bind(&db.description)
                .bind(&db.created_at)
                .bind(db_name)
                .execute(pg)
                .await?;
                Ok(result.rows_affected() > 0)
            }
            DbPool::Dummy => Err(sqlx::Error::Protocol(
                "Dummy pool: operation not supported".into(),
            )),
        }
    }

    /// 删除数据库/schema
    pub async fn delete(&self, db_name: &str) -> Result<bool, sqlx::Error> {
        match &self.pool {
            DbPool::MySql(mysql) => {
                let result = sqlx::query("DELETE FROM databases WHERE name = ?")
                    .bind(db_name)
                    .execute(mysql)
                    .await?;
                Ok(result.rows_affected() > 0)
            }
            DbPool::Postgres(pg) => {
                let result = sqlx::query("DELETE FROM databases WHERE name = $1")
                    .bind(db_name)
                    .execute(pg)
                    .await?;
                Ok(result.rows_affected() > 0)
            }
            DbPool::Dummy => Err(sqlx::Error::Protocol(
                "Dummy pool: operation not supported".into(),
            )),
        }
    }
}
