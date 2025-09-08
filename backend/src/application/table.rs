use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::table::{Table, TableId, Column, AddColumnRequest};

/// Response for table listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableListResponse {
    pub tables: Vec<TableSummary>,
}

/// Summary information about a table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSummary {
    pub id: TableId,
    pub column_count: usize,
    pub engine: Option<String>,
    pub charset: Option<String>,
}

impl From<&Table> for TableSummary {
    fn from(table: &Table) -> Self {
        Self {
            id: table.id.clone(),
            column_count: table.columns.len(),
            engine: table.engine.clone(),
            charset: table.charset.clone(),
        }
    }
}

/// Response for table details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableDetailResponse {
    pub table: Table,
}

/// Response for column operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnOperationResponse {
    pub status: String,
    pub message: String,
    pub column: Option<Column>,
}

impl ColumnOperationResponse {
    pub fn success(message: String, column: Option<Column>) -> Self {
        Self {
            status: "success".to_string(),
            message,
            column,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            status: "error".to_string(),
            message,
            column: None,
        }
    }
}

/// Application service trait for table operations
#[async_trait]
pub trait TableApplicationService: Send + Sync {
    /// Lists all tables in the database
    async fn list_tables(&self, database: Option<String>) -> anyhow::Result<TableListResponse>;
    
    /// Gets detailed information about a specific table
    async fn get_table_details(&self, table_id: &TableId) -> anyhow::Result<TableDetailResponse>;
    
    /// Adds a column to a table
    async fn add_column(&self, table_id: &TableId, request: AddColumnRequest) -> anyhow::Result<ColumnOperationResponse>;
    
    /// Removes a column from a table
    async fn remove_column(&self, table_id: &TableId, column_name: &str) -> anyhow::Result<ColumnOperationResponse>;
    
    /// Creates a new table
    async fn create_table(&self, table: Table) -> anyhow::Result<TableDetailResponse>;
    
    /// Drops a table
    async fn drop_table(&self, table_id: &TableId) -> anyhow::Result<ColumnOperationResponse>;
}

/// Implementation of the table application service
pub struct TableApplicationServiceImpl {
    // In a real implementation, this would contain database connections
    // For now, we'll use mock data
}

impl TableApplicationServiceImpl {
    pub fn new() -> Self {
        Self {}
    }

    /// Creates mock table data for demonstration
    fn create_mock_tables() -> Vec<Table> {
        let mut tables = Vec::new();

        // Create a sample users table
        let users_table_id = TableId::new("users".to_string());
        let mut users_table = Table::new(users_table_id);
        
        let id_column = Column::new("id".to_string(), crate::domain::table::ColumnType::Int)
            .primary_key()
            .auto_increment();
        let name_column = Column::new("name".to_string(), crate::domain::table::ColumnType::Varchar(Some(255)))
            .not_null();
        let email_column = Column::new("email".to_string(), crate::domain::table::ColumnType::Varchar(Some(255)));
        
        users_table = users_table.add_column(id_column).unwrap();
        users_table = users_table.add_column(name_column).unwrap();
        users_table = users_table.add_column(email_column).unwrap();
        users_table.engine = Some("InnoDB".to_string());
        users_table.charset = Some("utf8mb4".to_string());
        
        tables.push(users_table);

        // Create a sample orders table
        let orders_table_id = TableId::new("orders".to_string());
        let mut orders_table = Table::new(orders_table_id);
        
        let order_id_column = Column::new("order_id".to_string(), crate::domain::table::ColumnType::Int)
            .primary_key()
            .auto_increment();
        let user_id_column = Column::new("user_id".to_string(), crate::domain::table::ColumnType::Int)
            .not_null();
        let total_column = Column::new("total".to_string(), crate::domain::table::ColumnType::Decimal(Some((10, 2))));
        
        orders_table = orders_table.add_column(order_id_column).unwrap();
        orders_table = orders_table.add_column(user_id_column).unwrap();
        orders_table = orders_table.add_column(total_column).unwrap();
        orders_table.engine = Some("InnoDB".to_string());
        orders_table.charset = Some("utf8mb4".to_string());
        
        tables.push(orders_table);

        tables
    }

    /// Finds a table by ID in the mock data
    fn find_table_by_id(table_id: &TableId) -> Option<Table> {
        Self::create_mock_tables()
            .into_iter()
            .find(|table| table.id == *table_id)
    }
}

#[async_trait]
impl TableApplicationService for TableApplicationServiceImpl {
    async fn list_tables(&self, _database: Option<String>) -> anyhow::Result<TableListResponse> {
        let tables = Self::create_mock_tables();
        let table_summaries: Vec<TableSummary> = tables
            .iter()
            .map(|table| TableSummary::from(table))
            .collect();

        Ok(TableListResponse {
            tables: table_summaries,
        })
    }

    async fn get_table_details(&self, table_id: &TableId) -> anyhow::Result<TableDetailResponse> {
        match Self::find_table_by_id(table_id) {
            Some(table) => Ok(TableDetailResponse { table }),
            None => Err(anyhow::anyhow!("Table '{}' not found", table_id.full_name())),
        }
    }

    async fn add_column(&self, table_id: &TableId, request: AddColumnRequest) -> anyhow::Result<ColumnOperationResponse> {
        // Validate the request
        request.validate()
            .map_err(|e| anyhow::anyhow!("Invalid column request: {}", e))?;

        // In a real implementation, this would interact with the database
        // For now, we'll simulate the operation
        match Self::find_table_by_id(table_id) {
            Some(mut table) => {
                let column = request.to_column();
                match table.add_column(column.clone()) {
                    Ok(_) => Ok(ColumnOperationResponse::success(
                        format!("Column '{}' added successfully", request.column_name),
                        Some(column),
                    )),
                    Err(e) => Ok(ColumnOperationResponse::error(e)),
                }
            }
            None => Ok(ColumnOperationResponse::error(
                format!("Table '{}' not found", table_id.full_name())
            )),
        }
    }

    async fn remove_column(&self, table_id: &TableId, column_name: &str) -> anyhow::Result<ColumnOperationResponse> {
        // In a real implementation, this would interact with the database
        match Self::find_table_by_id(table_id) {
            Some(mut table) => {
                match table.remove_column(column_name) {
                    Ok(removed_column) => Ok(ColumnOperationResponse::success(
                        format!("Column '{}' deleted successfully", column_name),
                        Some(removed_column),
                    )),
                    Err(e) => Ok(ColumnOperationResponse::error(e)),
                }
            }
            None => Ok(ColumnOperationResponse::error(
                format!("Table '{}' not found", table_id.full_name())
            )),
        }
    }

    async fn create_table(&self, table: Table) -> anyhow::Result<TableDetailResponse> {
        // Validate the table
        table.validate()
            .map_err(|e| anyhow::anyhow!("Invalid table: {}", e))?;

        // In a real implementation, this would create the table in the database
        Ok(TableDetailResponse { table })
    }

    async fn drop_table(&self, table_id: &TableId) -> anyhow::Result<ColumnOperationResponse> {
        // In a real implementation, this would drop the table from the database
        match Self::find_table_by_id(table_id) {
            Some(_) => Ok(ColumnOperationResponse::success(
                format!("Table '{}' dropped successfully", table_id.full_name()),
                None,
            )),
            None => Ok(ColumnOperationResponse::error(
                format!("Table '{}' not found", table_id.full_name())
            )),
        }
    }
}

impl Default for TableApplicationServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_tables() {
        let service = TableApplicationServiceImpl::new();
        let result = service.list_tables(None).await.unwrap();
        
        assert!(!result.tables.is_empty());
        assert!(result.tables.iter().any(|t| t.id.name == "users"));
        assert!(result.tables.iter().any(|t| t.id.name == "orders"));
    }

    #[tokio::test]
    async fn test_get_table_details() {
        let service = TableApplicationServiceImpl::new();
        let table_id = TableId::new("users".to_string());
        
        let result = service.get_table_details(&table_id).await.unwrap();
        assert_eq!(result.table.id.name, "users");
        assert!(!result.table.columns.is_empty());
    }

    #[tokio::test]
    async fn test_add_column() {
        let service = TableApplicationServiceImpl::new();
        let table_id = TableId::new("users".to_string());
        
        let request = AddColumnRequest {
            column_name: "age".to_string(),
            column_type: "int".to_string(),
            nullable: Some(true),
            default_value: None,
        };

        let result = service.add_column(&table_id, request).await.unwrap();
        assert_eq!(result.status, "success");
        assert!(result.column.is_some());
        assert_eq!(result.column.unwrap().name, "age");
    }

    #[tokio::test]
    async fn test_remove_column() {
        let service = TableApplicationServiceImpl::new();
        let table_id = TableId::new("users".to_string());
        
        let result = service.remove_column(&table_id, "email").await.unwrap();
        assert_eq!(result.status, "success");
        assert!(result.column.is_some());
        assert_eq!(result.column.unwrap().name, "email");
    }

    #[tokio::test]
    async fn test_add_column_to_nonexistent_table() {
        let service = TableApplicationServiceImpl::new();
        let table_id = TableId::new("nonexistent".to_string());
        
        let request = AddColumnRequest {
            column_name: "test".to_string(),
            column_type: "int".to_string(),
            nullable: Some(true),
            default_value: None,
        };

        let result = service.add_column(&table_id, request).await.unwrap();
        assert_eq!(result.status, "error");
        assert!(result.message.contains("not found"));
    }
}
