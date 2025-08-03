use crate::domain::database::{Column, Table};
use crate::domain::database::DatabasePool;
use crate::domain::error::SqlEditorError;
use async_trait::async_trait;
use common::error::CommonError;
use std::sync::{Arc, Mutex};
use tracing::debug;
use crate::infrastructure::connection_store::ConnectionStore;
use sqlx::Row;

#[async_trait]
pub trait TableStoreOps: Send + Sync {
    async fn add(&self, table: Table, pool: DatabasePool) -> Result<(), SqlEditorError>;
    async fn list(&self, pool: DatabasePool, schema: &str) -> Result<Vec<Table>, SqlEditorError>;
    async fn get(&self, table_name: &str) -> Result<Option<Table>, SqlEditorError>;
    async fn update(&self, table_name: &str, table: Table) -> Result<bool, SqlEditorError>;
    async fn delete(&self, table_name: &str, pool: DatabasePool) -> Result<bool, SqlEditorError>;
    // async fn add_column(&self, table_name: &str, column: Column) -> Result<bool, SqlEditorError>;
    // async fn delete_column(
    //     &self,
    //     table_name: &str,
    //     column_name: &str,
    // ) -> Result<bool, SqlEditorError>;
}

/// Manages all in-memory tables with thread safety.
pub struct TableStore {
    pub tables: Arc<Mutex<Vec<Table>>>,
    pub connection_store: Arc<ConnectionStore>,
}

impl TableStore {
    pub fn new(connection_store: Arc<ConnectionStore>) -> Self {
        Self {
            tables: Arc::new(Mutex::new(Vec::new())),
            connection_store,
        }
    }
}

#[async_trait]
impl TableStoreOps for TableStore {
    async fn add(&self, table: Table, _pool: DatabasePool) -> Result<(), SqlEditorError> {
        debug!("TableStore::add table={:?}", table.table_name);
        self.tables
            .lock()
            .map_err(|e| SqlEditorError::InfraCommon(CommonError::LockError(e.to_string())))?
            .push(table);
        Ok(())
    }
    async fn list(&self, pool: DatabasePool, schema: &str) -> Result<Vec<Table>, SqlEditorError> {
        debug!("TableStore::list with sqlx, schema={}", schema);
        // 仅支持 MySQL，其他类型可扩展
        match pool {
            crate::domain::database::DatabasePool::MySql(mysql_pool) => {
                let sql = "SELECT TABLE_SCHEMA,TABLE_NAME,CREATE_TIME,TABLE_COMMENT FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = ?";
                let rows = sqlx::query(sql)
                    .bind(schema)
                    .fetch_all(&*mysql_pool)
                    .await
                    .map_err(|e| SqlEditorError::InfraCommon(CommonError::Other(e.to_string())))?;
                let tables = rows
                    .into_iter()
                    .map(|row| Table {
                        table_schema: row.get::<Option<String>, _>("TABLE_SCHEMA"),
                        table_name: row.get::<Option<String>, _>("TABLE_NAME"),
                        create_time: row.get::<Option<chrono::NaiveDateTime>, _>("CREATE_TIME"),
                        table_comment: row.get::<Option<String>, _>("TABLE_COMMENT"),
                    })
                    .collect();
                Ok(tables)
            }
            _ => Ok(vec![]),
        }
    }
    async fn get(&self, table_name: &str) -> Result<Option<Table>, SqlEditorError> {
        debug!("TableStore::get table_name={}", table_name);
        Ok(self
            .tables
            .lock()
            .map_err(|e| SqlEditorError::InfraCommon(CommonError::LockError(e.to_string())))?
            .iter()
            .find(|t| t.table_name.as_deref() == Some(table_name))
            .cloned())
    }
    async fn update(&self, table_name: &str, table: Table) -> Result<bool, SqlEditorError> {
        debug!("TableStore::update table_name={}", table_name);
        let mut tables = self
            .tables
            .lock()
            .map_err(|e| SqlEditorError::InfraCommon(CommonError::LockError(e.to_string())))?;
        if let Some(t) = tables.iter_mut().find(|t| t.table_name.as_deref() == Some(table_name)) {
            *t = table;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    async fn delete(&self, table_name: &str, _pool: DatabasePool) -> Result<bool, SqlEditorError> {
        debug!("TableStore::delete table_name={}", table_name);
        let mut tables = self
            .tables
            .lock()
            .map_err(|e| SqlEditorError::InfraCommon(CommonError::LockError(e.to_string())))?;
        let len_before = tables.len();
        tables.retain(|t| t.table_name.as_deref() != Some(table_name));
        Ok(len_before != tables.len())
    }
}
