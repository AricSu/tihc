use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::database::{Database, DatabaseConnection, DatabasePool, TableInfo, ColumnInfo, IndexInfo};
use crate::domain::error::SqlEditorError;
use crate::infrastructure::connection_store::{ConnectionStore, ConnectionOps};
use crate::infrastructure::database_store::{DatabaseStore, DatabaseBackend};
use crate::infrastructure::table_store::{TableStore, TableStoreOps};

/// Response for connection operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub id: u64,
    pub name: String,
    pub engine: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub database: Option<String>,
    pub use_tls: bool,
    pub created_at: String,
}

impl From<DatabaseConnection> for ConnectionResponse {
    fn from(connection: DatabaseConnection) -> Self {
        Self {
            id: connection.id,
            name: connection.name,
            engine: connection.engine,
            host: connection.host,
            port: connection.port,
            username: connection.username,
            database: connection.database,
            use_tls: connection.use_tls,
            created_at: connection.created_at,
        }
    }
}

/// Response for connection listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionListResponse {
    pub connections: Vec<ConnectionResponse>,
}

impl From<Vec<DatabaseConnection>> for ConnectionListResponse {
    fn from(connections: Vec<DatabaseConnection>) -> Self {
        Self {
            connections: connections.into_iter().map(ConnectionResponse::from).collect(),
        }
    }
}

/// Response for connection testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTestResponse {
    pub success: bool,
    pub message: String,
    pub latency_ms: Option<u64>,
    pub server_version: Option<String>,
}

impl ConnectionTestResponse {
    pub fn success(latency_ms: u64, server_version: Option<String>) -> Self {
        Self {
            success: true,
            message: "Connection successful".to_string(),
            latency_ms: Some(latency_ms),
            server_version,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            latency_ms: None,
            server_version: None,
        }
    }
}

/// Response for database operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseListResponse {
    pub databases: Vec<Database>,
    pub connection_id: u64,
}

/// Response for table operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableListResponse {
    pub tables: Vec<TableInfo>,
    pub connection_id: u64,
    pub database: Option<String>,
}

/// Response for column operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnListResponse {
    pub columns: Vec<ColumnInfo>,
    pub connection_id: u64,
    pub schema: Option<String>,
    pub table: Option<String>,
}

/// Response for index operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexListResponse {
    pub indexes: Vec<IndexInfo>,
    pub connection_id: u64,
    pub schema: Option<String>,
    pub table: Option<String>,
}

/// Response for SQL execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlExecutionResponse {
    pub success: bool,
    pub rows_affected: Option<u64>,
    pub execution_time_ms: u64,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// SQL Editor application service trait
#[async_trait]
pub trait SqlEditorApplicationService: Send + Sync {
    // Connection management
    async fn create_connection(&self, connection: DatabaseConnection) -> Result<ConnectionResponse, SqlEditorError>;
    async fn get_connection(&self, id: u64) -> Result<Option<ConnectionResponse>, SqlEditorError>;
    async fn list_connections(&self) -> Result<ConnectionListResponse, SqlEditorError>;
    async fn update_connection(&self, id: u64, connection: DatabaseConnection) -> Result<ConnectionResponse, SqlEditorError>;
    async fn delete_connection(&self, id: u64) -> Result<bool, SqlEditorError>;
    async fn test_connection(&self, connection: &DatabaseConnection) -> Result<ConnectionTestResponse, SqlEditorError>;
    
    // Database operations
    async fn list_databases(&self, connection_id: u64) -> Result<DatabaseListResponse, SqlEditorError>;
    async fn create_database(&self, connection_id: u64, database: &Database) -> Result<(), SqlEditorError>;
    async fn delete_database(&self, connection_id: u64, database_name: &str) -> Result<bool, SqlEditorError>;
    
    // Table operations
    async fn list_tables(&self, connection_id: u64, schema_name: &str) -> Result<TableListResponse, SqlEditorError>;
    async fn create_table(&self, connection_id: u64, table: TableInfo) -> Result<(), SqlEditorError>;
    async fn delete_table(&self, connection_id: u64, table_name: &str) -> Result<bool, SqlEditorError>;
    
    // Column operations
    async fn list_columns(&self, connection_id: u64, schema_name: &str, table_name: &str) -> Result<ColumnListResponse, SqlEditorError>;
    
    // Index operations
    async fn list_indexes(&self, connection_id: u64, schema_name: &str, table_name: &str) -> Result<IndexListResponse, SqlEditorError>;
    
    // SQL execution
    async fn execute_sql(&self, connection_id: u64, sql: &str) -> Result<SqlExecutionResponse, SqlEditorError>;
}

/// Implementation of the SQL Editor application service
pub struct SqlEditorApplicationServiceImpl {
    connection_store: Arc<ConnectionStore>,
    database_store: Arc<DatabaseStore>,
    table_store: Arc<TableStore>,
}

impl SqlEditorApplicationServiceImpl {
    pub fn new() -> Self {
        let connection_store = Arc::new(ConnectionStore::new());
        let database_store = Arc::new(DatabaseStore::new_dummy(Arc::clone(&connection_store)));
        let table_store = Arc::new(TableStore::new(Arc::clone(&connection_store)));
        
        Self {
            connection_store,
            database_store,
            table_store,
        }
    }
    
    pub fn with_stores(
        connection_store: Arc<ConnectionStore>,
        database_store: Arc<DatabaseStore>,
        table_store: Arc<TableStore>,
    ) -> Self {
        Self {
            connection_store,
            database_store,
            table_store,
        }
    }
}

#[async_trait]
impl SqlEditorApplicationService for SqlEditorApplicationServiceImpl {
    async fn create_connection(&self, connection: DatabaseConnection) -> Result<ConnectionResponse, SqlEditorError> {
        self.connection_store.add_connection(connection.clone()).await?;
        Ok(ConnectionResponse::from(connection))
    }

    async fn get_connection(&self, id: u64) -> Result<Option<ConnectionResponse>, SqlEditorError> {
        match self.connection_store.get_connection(id).await? {
            Some(connection) => Ok(Some(ConnectionResponse::from(connection))),
            None => Ok(None),
        }
    }

    async fn list_connections(&self) -> Result<ConnectionListResponse, SqlEditorError> {
        let connections = self.connection_store.list_connection().await?;
        Ok(ConnectionListResponse::from(connections))
    }

    async fn update_connection(&self, id: u64, connection: DatabaseConnection) -> Result<ConnectionResponse, SqlEditorError> {
        let updated = self.connection_store.update_connection(id, connection).await?;
        Ok(ConnectionResponse::from(updated))
    }

    async fn delete_connection(&self, id: u64) -> Result<bool, SqlEditorError> {
        self.connection_store.delete_connection(id).await
    }

    async fn test_connection(&self, connection: &DatabaseConnection) -> Result<ConnectionTestResponse, SqlEditorError> {
        self.connection_store.test_connection(connection).await
    }

    async fn list_databases(&self, connection_id: u64) -> Result<DatabaseListResponse, SqlEditorError> {
        let pool = self.connection_store.get_pool(connection_id);
        if pool.is_none() {
            return Err(SqlEditorError::Other("Connection not found".to_string()));
        }
        
        let backend: Arc<dyn DatabaseBackend> = match &pool {
            Some(DatabasePool::MySql(mysql_pool)) => {
                Arc::new(crate::infrastructure::database_store::MySqlBackend {
                    pool: Arc::clone(mysql_pool),
                })
            }
            _ => Arc::new(crate::infrastructure::database_store::DummyBackend),
        };
        
        let databases = backend.list(pool.unwrap()).await?;
        Ok(DatabaseListResponse {
            databases,
            connection_id,
        })
    }

    async fn create_database(&self, _connection_id: u64, database: &Database) -> Result<(), SqlEditorError> {
        self.database_store.backend.add(database).await
    }

    async fn delete_database(&self, _connection_id: u64, database_name: &str) -> Result<bool, SqlEditorError> {
        self.database_store.backend.delete(database_name).await
    }

    async fn list_tables(&self, connection_id: u64, schema_name: &str) -> Result<TableListResponse, SqlEditorError> {
        let pool = self.table_store.connection_store.get_pool(connection_id);
        if pool.is_none() {
            return Err(SqlEditorError::Other("Connection not found".to_string()));
        }
        
        let tables = self.table_store.list(pool.unwrap(), schema_name).await?;
        Ok(TableListResponse {
            tables,
            connection_id,
            database: Some(schema_name.to_string()),
        })
    }

    async fn create_table(&self, connection_id: u64, table: TableInfo) -> Result<(), SqlEditorError> {
        let pool = self.table_store.connection_store.get_pool(connection_id);
        if pool.is_none() {
            return Err(SqlEditorError::Other("Connection not found".to_string()));
        }
        
        self.table_store.add(table, pool.unwrap()).await
    }

    async fn delete_table(&self, connection_id: u64, table_name: &str) -> Result<bool, SqlEditorError> {
        let pool = self.table_store.connection_store.get_pool(connection_id);
        if pool.is_none() {
            return Err(SqlEditorError::Other("Connection not found".to_string()));
        }
        
        self.table_store.delete(table_name, pool.unwrap()).await
    }

    async fn list_columns(&self, connection_id: u64, schema_name: &str, table_name: &str) -> Result<ColumnListResponse, SqlEditorError> {
        let pool = self.table_store.connection_store.get_pool(connection_id);
        if pool.is_none() {
            return Err(SqlEditorError::Other("Connection not found".to_string()));
        }
        
        let columns = self.table_store.list_columns(pool.unwrap(), schema_name, table_name).await?;
        Ok(ColumnListResponse {
            columns,
            connection_id,
            schema: Some(schema_name.to_string()),
            table: Some(table_name.to_string()),
        })
    }

    async fn list_indexes(&self, connection_id: u64, schema_name: &str, table_name: &str) -> Result<IndexListResponse, SqlEditorError> {
        let pool = self.table_store.connection_store.get_pool(connection_id);
        if pool.is_none() {
            return Err(SqlEditorError::Other("Connection not found".to_string()));
        }
        
        let indexes = self.table_store.list_indexes(pool.unwrap(), schema_name, table_name).await?;
        Ok(IndexListResponse {
            indexes,
            connection_id,
            schema: Some(schema_name.to_string()),
            table: Some(table_name.to_string()),
        })
    }

    async fn execute_sql(&self, connection_id: u64, sql: &str) -> Result<SqlExecutionResponse, SqlEditorError> {
        let pool = self.connection_store.get_pool(connection_id);
        if pool.is_none() {
            return Err(SqlEditorError::Other("Connection not found".to_string()));
        }
        
        let backend: Arc<dyn DatabaseBackend> = match &pool {
            Some(DatabasePool::MySql(mysql_pool)) => {
                Arc::new(crate::infrastructure::database_store::MySqlBackend {
                    pool: Arc::clone(mysql_pool),
                })
            }
            _ => Arc::new(crate::infrastructure::database_store::DummyBackend),
        };
        
        let start = std::time::Instant::now();
        match backend.execute_sql(sql).await {
            Ok(result) => {
                let execution_time = start.elapsed().as_millis() as u64;
                Ok(SqlExecutionResponse {
                    success: true,
                    rows_affected: None, // Would be extracted from result
                    execution_time_ms: execution_time,
                    data: Some(result),
                    error: None,
                })
            }
            Err(e) => {
                let execution_time = start.elapsed().as_millis() as u64;
                Ok(SqlExecutionResponse {
                    success: false,
                    rows_affected: None,
                    execution_time_ms: execution_time,
                    data: None,
                    error: Some(e.to_string()),
                })
            }
        }
    }
}

impl Default for SqlEditorApplicationServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}