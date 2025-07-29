use crate::domain::database::Table;
use std::sync::{Arc, Mutex};

pub struct TableStore {
    pub tables: Arc<Mutex<Vec<Table>>>,
}

impl TableStore {
    pub fn new() -> Self {
        Self {
            tables: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, table: Table) {
        self.tables.lock().unwrap().push(table);
    }

    pub fn list(&self) -> Vec<Table> {
        self.tables.lock().unwrap().clone()
    }

    pub fn add_column(&self, table_name: &str, column: crate::domain::database::Column) -> bool {
        let mut tables = self.tables.lock().unwrap();
        if let Some(table) = tables.iter_mut().find(|t| t.name == table_name) {
            table.columns.push(column);
            return true;
        }
        false
    }

    pub fn delete_column(&self, table_name: &str, column_name: &str) -> bool {
        let mut tables = self.tables.lock().unwrap();
        if let Some(table) = tables.iter_mut().find(|t| t.name == table_name) {
            let len_before = table.columns.len();
            table.columns.retain(|c| c.name != column_name);
            return len_before != table.columns.len();
        }
        false
    }
}
