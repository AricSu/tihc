// Application Services - 应用服务接口定义

use crate::domain::shared::{DomainResult, PagedResult, Pagination};
use async_trait::async_trait;

/// 编辑器应用服务接口
#[async_trait]
pub trait EditorApplicationService: Send + Sync {
    async fn create_query(&self, request: CreateQueryRequest) -> DomainResult<QueryResponse>;
    async fn execute_query(
        &self,
        request: ExecuteQueryRequest,
    ) -> DomainResult<QueryResultResponse>;
    async fn get_query(&self, query_id: &str) -> DomainResult<QueryResponse>;
    async fn list_queries(
        &self,
        database_id: &str,
        pagination: Pagination,
    ) -> DomainResult<PagedResult<QueryResponse>>;
    async fn update_query(&self, request: UpdateQueryRequest) -> DomainResult<QueryResponse>;
    async fn delete_query(&self, query_id: &str) -> DomainResult<()>;
}

/// 数据库应用服务接口
#[async_trait]
pub trait DatabaseApplicationService: Send + Sync {
    async fn create_connection(
        &self,
        request: CreateConnectionRequest,
    ) -> DomainResult<ConnectionResponse>;
    async fn test_connection(&self, connection_id: &str) -> DomainResult<bool>;
    async fn get_connection(&self, connection_id: &str) -> DomainResult<ConnectionResponse>;
    async fn list_connections(&self) -> DomainResult<Vec<ConnectionResponse>>;
    async fn update_connection(
        &self,
        request: UpdateConnectionRequest,
    ) -> DomainResult<ConnectionResponse>;
    async fn delete_connection(&self, connection_id: &str) -> DomainResult<()>;
    async fn get_tables(&self, connection_id: &str) -> DomainResult<Vec<TableResponse>>;
    async fn get_table_info(
        &self,
        connection_id: &str,
        table_name: &str,
    ) -> DomainResult<TableDetailResponse>;
}

// 请求和响应类型
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateQueryRequest {
    pub database_id: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteQueryRequest {
    pub query_id: String,
    pub timeout_seconds: Option<u64>,
    pub max_rows: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQueryRequest {
    pub query_id: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    pub id: String,
    pub database_id: String,
    pub content: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct QueryResultResponse {
    pub query_id: String,
    pub columns: Vec<String>,
    pub rows: Vec<std::collections::HashMap<String, serde_json::Value>>,
    pub affected_rows: Option<u64>,
    pub execution_time_ms: u64,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub connection_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateConnectionRequest {
    pub connection_id: String,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub database_name: String,
    pub connection_type: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableResponse {
    pub name: String,
    pub schema: Option<String>,
    pub row_count: Option<u64>,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableDetailResponse {
    pub name: String,
    pub schema: Option<String>,
    pub row_count: Option<u64>,
    pub size_bytes: Option<u64>,
    pub columns: Vec<ColumnResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnResponse {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub default_value: Option<String>,
    pub comment: Option<String>,
}
