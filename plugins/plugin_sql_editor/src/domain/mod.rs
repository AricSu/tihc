pub mod database;
pub mod error;
pub mod sql;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionListResult {
    pub data: Vec<crate::domain::database::DatabaseConnection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableListResult {
    pub data: Vec<crate::domain::database::Table>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResult {
    pub status: String,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlQueryResult {
    pub status: String,
    pub data: Option<Vec<serde_json::Value>>,
    pub message: Option<String>,
}
