// --- Imports ---
use crate::domain::database::{Database, DatabaseConnection};
use crate::domain::error::SqlEditorError;
use crate::domain::sql::{SqlMessage, SqlQueryResult};
use sqlx::{MySqlPool, PgPool, Connection};
use tracing::{debug, warn};

// --- SQL Constants ---
const MYSQL_INSERT: &str = "INSERT INTO databases (name, description, created_at) VALUES (?, ?, ?)";
const POSTGRES_INSERT: &str =
    "INSERT INTO databases (name, description, created_at) VALUES ($1, $2, $3)";
const MYSQL_SELECT_ALL: &str = "SELECT name, description, created_at FROM databases";
const POSTGRES_SELECT_ALL: &str = "SELECT name, description, created_at FROM databases";
const MYSQL_SELECT_ONE: &str = "SELECT name, description, created_at FROM databases WHERE name = ?";
const POSTGRES_SELECT_ONE: &str =
    "SELECT name, description, created_at FROM databases WHERE name = $1";
const MYSQL_UPDATE: &str = "UPDATE databases SET description = ?, created_at = ? WHERE name = ?";
const POSTGRES_UPDATE: &str =
    "UPDATE databases SET description = $1, created_at = $2 WHERE name = $3";
const MYSQL_DELETE: &str = "DELETE FROM databases WHERE name = ?";
const POSTGRES_DELETE: &str = "DELETE FROM databases WHERE name = $1";

// --- Helper ---
fn dummy_err() -> sqlx::Error {
    sqlx::Error::Protocol("Dummy pool: operation not supported".into())
}

// --- Pool Enum ---
#[derive(Clone)]
pub enum DbPool {
    MySql(MySqlPool),
    Postgres(PgPool),
    Dummy,
}

// --- Store Struct ---
pub struct DatabaseStore {
    pub pool: DbPool,
}

// --- Constructors ---
impl DatabaseStore {
    /// 构造 MySQL 数据库存储
    pub fn with_mysql(conn: &DatabaseConnection) -> Self {
        debug!(
            "with_mysql: host={}, port={}, db={}",
            conn.host,
            conn.port,
            conn.database.as_deref().unwrap_or_default()
        );
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            conn.username,
            conn.password.as_deref().unwrap_or_default(),
            conn.host,
            conn.port,
            conn.database.as_deref().unwrap_or_default()
        );
        let pool = MySqlPool::connect_lazy(&url).expect("Failed to create MySQL pool");
        Self {
            pool: DbPool::MySql(pool),
        }
    }

    /// 构造 Postgres 数据库存储
    pub fn with_postgres(conn: &DatabaseConnection) -> Self {
        debug!(
            "with_postgres: host={}, port={}, db={}",
            conn.host,
            conn.port,
            conn.database.as_deref().unwrap_or_default()
        );
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            conn.username,
            conn.password.as_deref().unwrap_or_default(),
            conn.host,
            conn.port,
            conn.database.as_deref().unwrap_or_default()
        );
        let pool = PgPool::connect_lazy(&url).expect("Failed to create Postgres pool");
        Self {
            pool: DbPool::Postgres(pool),
        }
    }

    /// 构造 Dummy 数据库存储
    pub fn dummy() -> Self {
        warn!("DatabaseStore::dummy() called, pool=Dummy");
        Self {
            pool: DbPool::Dummy,
        }
    }

    /// 构造通用
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // --- 连接测试 ---
    /// 测试数据库连接是否可用（不创建连接池，只做一次性连接验证）
    pub async fn test_connection(&self, conn: &DatabaseConnection) -> Result<bool, sqlx::Error> {
        debug!("test_connection: type={:?}, host={}, port={}, db={}", self.pool_type(), conn.host, conn.port, conn.database.as_deref().unwrap_or_default());
        match self.pool_type() {
            "MySql" => {
                let url = format!(
                    "mysql://{}:{}@{}:{}/{}",
                    conn.username,
                    conn.password.as_deref().unwrap_or_default(),
                    conn.host,
                    conn.port,
                    conn.database.as_deref().unwrap_or_default()
                );
                match sqlx::MySqlConnection::connect(&url).await {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            "Postgres" => {
                let url = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    conn.username,
                    conn.password.as_deref().unwrap_or_default(),
                    conn.host,
                    conn.port,
                    conn.database.as_deref().unwrap_or_default()
                );
                match sqlx::PgConnection::connect(&url).await {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            _ => Err(dummy_err()),
        }
    }

    // --- 业务方法 ---
    /// 添加数据库/schema
    pub async fn add(&self, db: &Database) -> Result<(), sqlx::Error> {
        debug!("add: pool={:?}, db={}", self.pool_type(), db.name);
        match &self.pool {
            DbPool::MySql(mysql) => {
                sqlx::query(MYSQL_INSERT)
                    .bind(&db.name)
                    .bind(&db.description)
                    .bind(&db.created_at)
                    .execute(mysql)
                    .await?;
                Ok(())
            }
            DbPool::Postgres(pg) => {
                sqlx::query(POSTGRES_INSERT)
                    .bind(&db.name)
                    .bind(&db.description)
                    .bind(&db.created_at)
                    .execute(pg)
                    .await?;
                Ok(())
            }
            DbPool::Dummy => Err(dummy_err()),
        }
    }

    /// 列出所有数据库/schema
    pub async fn list(&self) -> Result<Vec<Database>, sqlx::Error> {
        debug!("list: pool={:?}", self.pool_type());
        match &self.pool {
            DbPool::MySql(mysql) => {
                let rows = sqlx::query_as::<_, Database>(MYSQL_SELECT_ALL)
                    .fetch_all(mysql)
                    .await?;
                Ok(rows)
            }
            DbPool::Postgres(pg) => {
                let rows = sqlx::query_as::<_, Database>(POSTGRES_SELECT_ALL)
                    .fetch_all(pg)
                    .await?;
                Ok(rows)
            }
            DbPool::Dummy => Err(dummy_err()),
        }
    }

    /// 按名查找数据库/schema
    pub async fn get(&self, db_name: &str) -> Result<Option<Database>, sqlx::Error> {
        debug!("get: pool={:?}, db_name={}", self.pool_type(), db_name);
        match &self.pool {
            DbPool::MySql(mysql) => {
                let row = sqlx::query_as::<_, Database>(MYSQL_SELECT_ONE)
                    .bind(db_name)
                    .fetch_optional(mysql)
                    .await?;
                Ok(row)
            }
            DbPool::Postgres(pg) => {
                let row = sqlx::query_as::<_, Database>(POSTGRES_SELECT_ONE)
                    .bind(db_name)
                    .fetch_optional(pg)
                    .await?;
                Ok(row)
            }
            DbPool::Dummy => Err(dummy_err()),
        }
    }

    /// 更新数据库/schema（全量覆盖）
    pub async fn update(&self, db_name: &str, db: &Database) -> Result<bool, sqlx::Error> {
        debug!("update: pool={:?}, db_name={}", self.pool_type(), db_name);
        match &self.pool {
            DbPool::MySql(mysql) => {
                let result = sqlx::query(MYSQL_UPDATE)
                    .bind(&db.description)
                    .bind(&db.created_at)
                    .bind(db_name)
                    .execute(mysql)
                    .await?;
                Ok(result.rows_affected() > 0)
            }
            DbPool::Postgres(pg) => {
                let result = sqlx::query(POSTGRES_UPDATE)
                    .bind(&db.description)
                    .bind(&db.created_at)
                    .bind(db_name)
                    .execute(pg)
                    .await?;
                Ok(result.rows_affected() > 0)
            }
            DbPool::Dummy => Err(dummy_err()),
        }
    }

    /// 删除数据库/schema
    pub async fn delete(&self, db_name: &str) -> Result<bool, sqlx::Error> {
        debug!("delete: pool={:?}, db_name={}", self.pool_type(), db_name);
        match &self.pool {
            DbPool::MySql(mysql) => {
                let result = sqlx::query(MYSQL_DELETE)
                    .bind(db_name)
                    .execute(mysql)
                    .await?;
                Ok(result.rows_affected() > 0)
            }
            DbPool::Postgres(pg) => {
                let result = sqlx::query(POSTGRES_DELETE)
                    .bind(db_name)
                    .execute(pg)
                    .await?;
                Ok(result.rows_affected() > 0)
            }
            DbPool::Dummy => Err(dummy_err()),
        }
    }

    // --- SQL 执行 ---
    /// Executes a SQL query and collects columns, types, rows, and messages.
    pub async fn execute_sql(&self, sql: &str) -> Result<SqlQueryResult, SqlEditorError> {
        debug!("execute_sql: pool={:?}, sql={}", self.pool_type(), sql);
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
}

// --- Pool type helper ---

// 放在文件最外层，避免嵌套 impl warning
impl DatabaseStore {
    fn pool_type(&self) -> &'static str {
        match &self.pool {
            DbPool::MySql(_) => "MySql",
            DbPool::Postgres(_) => "Postgres",
            DbPool::Dummy => "Dummy",
        }
    }
}
