// Database Domain - 数据库管理领域模型

use crate::domain::shared::{DomainError, DomainResult, DatabaseId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 数据库连接实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnection {
    pub id: DatabaseId,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub database_name: String,
    pub connection_type: DatabaseType,
    pub config: DatabaseConfig,
    pub status: ConnectionStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 数据库类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseType {
    MySQL,
    PostgreSQL,
    SQLite,
    TiDB,
}

/// 连接状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Active,
    Inactive,
    Error,
    Testing,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub ssl_enabled: bool,
    pub connection_timeout: u64,
    pub max_connections: u32,
    pub custom_properties: HashMap<String, String>,
}

/// 数据库连接聚合根
impl DatabaseConnection {
    pub fn new(
        name: String,
        host: String,
        port: u16,
        username: String,
        database_name: String,
        connection_type: DatabaseType,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: DatabaseId::new(uuid::Uuid::new_v4().to_string()),
            name,
            host,
            port,
            username,
            database_name,
            connection_type,
            config: DatabaseConfig::default(),
            status: ConnectionStatus::Inactive,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_config(&mut self, config: DatabaseConfig) -> DomainResult<()> {
        self.config = config;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }
    
    pub fn activate(&mut self) -> DomainResult<()> {
        match self.status {
            ConnectionStatus::Inactive | ConnectionStatus::Error => {
                self.status = ConnectionStatus::Active;
                self.updated_at = chrono::Utc::now();
                Ok(())
            }
            _ => Err(DomainError::BusinessRuleViolation {
                rule: "只有非活跃或错误状态的连接才能激活".to_string(),
            }),
        }
    }
    
    pub fn deactivate(&mut self) -> DomainResult<()> {
        if self.status == ConnectionStatus::Active {
            self.status = ConnectionStatus::Inactive;
            self.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(DomainError::BusinessRuleViolation {
                rule: "只有活跃状态的连接才能停用".to_string(),
            })
        }
    }
    
    pub fn mark_error(&mut self) -> DomainResult<()> {
        self.status = ConnectionStatus::Error;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            ssl_enabled: false,
            connection_timeout: 30,
            max_connections: 10,
            custom_properties: HashMap::new(),
        }
    }
}

/// 数据库表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub schema: Option<String>,
    pub row_count: Option<u64>,
    pub size_bytes: Option<u64>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub engine: Option<String>,
    pub comment: Option<String>,
}

/// 数据库列信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub default_value: Option<String>,
    pub comment: Option<String>,
}

/// 数据库连接仓储接口
#[async_trait::async_trait]
pub trait DatabaseConnectionRepository {
    async fn save(&self, connection: &DatabaseConnection) -> DomainResult<()>;
    async fn find_by_id(&self, id: &DatabaseId) -> DomainResult<Option<DatabaseConnection>>;
    async fn find_all(&self) -> DomainResult<Vec<DatabaseConnection>>;
    async fn delete(&self, id: &DatabaseId) -> DomainResult<()>;
}

/// 数据库元数据服务接口
#[async_trait::async_trait]
pub trait DatabaseMetadataService {
    async fn test_connection(&self, connection: &DatabaseConnection) -> DomainResult<bool>;
    async fn get_tables(&self, connection: &DatabaseConnection) -> DomainResult<Vec<TableInfo>>;
    async fn get_table_columns(
        &self,
        connection: &DatabaseConnection,
        table_name: &str,
    ) -> DomainResult<Vec<ColumnInfo>>;
    async fn get_database_size(&self, connection: &DatabaseConnection) -> DomainResult<u64>;
}

/// 数据库领域服务
pub struct DatabaseDomainService {
    connection_repository: Box<dyn DatabaseConnectionRepository + Send + Sync>,
    metadata_service: Box<dyn DatabaseMetadataService + Send + Sync>,
}

impl DatabaseDomainService {
    pub fn new(
        connection_repository: Box<dyn DatabaseConnectionRepository + Send + Sync>,
        metadata_service: Box<dyn DatabaseMetadataService + Send + Sync>,
    ) -> Self {
        Self {
            connection_repository,
            metadata_service,
        }
    }
    
    pub async fn test_and_activate_connection(
        &self,
        connection_id: &DatabaseId,
    ) -> DomainResult<()> {
        let mut connection = self
            .connection_repository
            .find_by_id(connection_id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: format!("数据库连接 {}", connection_id.as_str()),
            })?;
        
        // 测试连接
        if self.metadata_service.test_connection(&connection).await? {
            connection.activate()?;
        } else {
            connection.mark_error()?;
        }
        
        self.connection_repository.save(&connection).await?;
        Ok(())
    }
}
