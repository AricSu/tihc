#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Database {
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub comment: Option<String>,
    pub primary_key: Option<Vec<String>>,
    pub unique_keys: Option<Vec<Vec<String>>>,
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
