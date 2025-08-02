use crate::domain::database::{Column, Table};
use std::sync::{Arc, Mutex};

/// Manages all in-memory tables with thread safety.
pub struct TableStore {
    /// Shared, thread-safe vector of tables.
    pub tables: Arc<Mutex<Vec<Table>>>,
}

impl TableStore {
    /// Creates a new, empty TableStore.
    pub fn new() -> Self {
        Self {
            tables: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds a table to the store.
    pub fn add(&self, table: Table) {
        self.tables
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock tables."))
            .unwrap()
            .push(table);
    }

    /// Returns a clone of all tables.
    pub fn list(&self) -> Vec<Table> {
        self.tables
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock tables."))
            .unwrap()
            .clone()
    }

    /// Finds a table by name.
    pub fn get(&self, table_name: &str) -> Option<Table> {
        self.tables
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock tables."))
            .unwrap()
            .iter()
            .find(|t| t.name == table_name)
            .cloned()
    }

    /// Updates a table by name. Returns true if updated.
    pub fn update(&self, table_name: &str, table: Table) -> bool {
        let mut tables = self
            .tables
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock tables."))
            .unwrap();
        if let Some(t) = tables.iter_mut().find(|t| t.name == table_name) {
            *t = table;
            return true;
        }
        false
    }

    /// Deletes a table by name. Returns true if deleted.
    pub fn delete(&self, table_name: &str) -> bool {
        let mut tables = self
            .tables
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock tables."))
            .unwrap();
        let len_before = tables.len();
        tables.retain(|t| t.name != table_name);
        len_before != tables.len()
    }

    /// Adds a column to the specified table. Returns true if added.
    pub fn add_column(&self, table_name: &str, column: Column) -> bool {
        let mut tables = self
            .tables
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock tables."))
            .unwrap();
        let Some(table) = tables.iter_mut().find(|t| t.name == table_name) else {
            return false;
        };
        table.columns.push(column);
        true
    }

    /// Deletes a column from the specified table. Returns true if deleted.
    pub fn delete_column(&self, table_name: &str, column_name: &str) -> bool {
        let mut tables = self
            .tables
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock tables."))
            .unwrap();
        let Some(table) = tables.iter_mut().find(|t| t.name == table_name) else {
            return false;
        };
        let len_before = table.columns.len();
        table.columns.retain(|c| c.name != column_name);
        len_before != table.columns.len()
    }
}
