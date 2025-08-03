use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, PgPool, SqlitePool};
use std::sync::Arc;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Database {
    #[sqlx(rename = "SCHEMA_NAME")]
    pub schema_name: String,
    #[sqlx(rename = "DEFAULT_COLLATION_NAME")]
    pub default_collation_name: String,
    #[sqlx(rename = "DEFAULT_CHARACTER_SET_NAME")]
    pub default_character_set_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConnection {
    pub id: u64,
    pub name: String,
    /// 数据库类型，如 "mysql" 或 "postgres"
    pub engine: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub use_tls: bool,
    pub ca_cert_path: Option<String>,
    pub created_at: String,
    #[serde(skip)]
    pub pool: Option<DatabasePool>,
}

// 支持多种数据库类型的连接池
#[derive(Debug, Clone)]
pub enum DatabasePool {
    MySql(Arc<MySqlPool>),
    Postgres(Arc<PgPool>),
    Sqlite(Arc<SqlitePool>),
    // 其它类型可扩展
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub table_schema: Option<String>,
    pub table_name: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub table_comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub column_type: String,
    pub nullable: bool,
    pub default: Option<String>,
    pub comment: Option<String>,
    pub is_primary: bool,
    pub is_unique: bool,
}
