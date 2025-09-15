// Database Application Service Implementation
// 数据库应用服务实现

use crate::application::services::{
    ConnectionResponse, CreateConnectionRequest, DatabaseApplicationService, TableDetailResponse,
    TableResponse, UpdateConnectionRequest,
};
use crate::domain::shared::{DomainError, DomainResult};
use async_trait::async_trait;

/// 数据库应用服务的实现
pub struct DatabaseApplicationServiceImpl {}

impl DatabaseApplicationServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl DatabaseApplicationService for DatabaseApplicationServiceImpl {
    async fn create_connection(
        &self,
        _request: CreateConnectionRequest,
    ) -> DomainResult<ConnectionResponse> {
        Err(DomainError::InternalError {
            message: "Database service temporarily disabled".to_string(),
        })
    }

    async fn test_connection(&self, _connection_id: &str) -> DomainResult<bool> {
        Err(DomainError::InternalError {
            message: "Database service temporarily disabled".to_string(),
        })
    }

    async fn get_connection(&self, _connection_id: &str) -> DomainResult<ConnectionResponse> {
        Err(DomainError::InternalError {
            message: "Database service temporarily disabled".to_string(),
        })
    }

    async fn list_connections(&self) -> DomainResult<Vec<ConnectionResponse>> {
        Err(DomainError::InternalError {
            message: "Database service temporarily disabled".to_string(),
        })
    }

    async fn update_connection(
        &self,
        _request: UpdateConnectionRequest,
    ) -> DomainResult<ConnectionResponse> {
        Err(DomainError::InternalError {
            message: "Database service temporarily disabled".to_string(),
        })
    }

    async fn delete_connection(&self, _connection_id: &str) -> DomainResult<()> {
        Err(DomainError::InternalError {
            message: "Database service temporarily disabled".to_string(),
        })
    }

    async fn get_tables(&self, _connection_id: &str) -> DomainResult<Vec<TableResponse>> {
        Err(DomainError::InternalError {
            message: "Database service temporarily disabled".to_string(),
        })
    }

    async fn get_table_info(
        &self,
        _connection_id: &str,
        _table_name: &str,
    ) -> DomainResult<TableDetailResponse> {
        Err(DomainError::InternalError {
            message: "Database service temporarily disabled".to_string(),
        })
    }
}
