use crate::domain::database::{Column, Table};
use std::sync::{Arc, Mutex};

/// TableStore 负责所有表的内存管理与并发安全
pub struct TableStore {
    pub tables: Arc<Mutex<Vec<Table>>>,
}

impl TableStore {
    /// 创建新的 TableStore
    pub fn new() -> Self {
        Self {
            tables: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 添加表
    pub fn add(&self, table: Table) {
        self.tables.lock().unwrap().push(table);
    }

    /// 列出所有表
    pub fn list(&self) -> Vec<Table> {
        self.tables.lock().unwrap().clone()
    }

    /// 按表名查找表
    pub fn get(&self, table_name: &str) -> Option<Table> {
        self.tables
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.name == table_name)
            .cloned()
    }

    /// 更新表（全量覆盖）
    pub fn update(&self, table_name: &str, table: Table) -> bool {
        let mut tables = self.tables.lock().unwrap();
        if let Some(t) = tables.iter_mut().find(|t| t.name == table_name) {
            *t = table;
            return true;
        }
        false
    }

    /// 删除表
    pub fn delete(&self, table_name: &str) -> bool {
        let mut tables = self.tables.lock().unwrap();
        let len_before = tables.len();
        tables.retain(|t| t.name != table_name);
        len_before != tables.len()
    }

    /// 添加列到指定表
    pub fn add_column(&self, table_name: &str, column: Column) -> bool {
        let mut tables = self.tables.lock().unwrap();
        if let Some(table) = tables.iter_mut().find(|t| t.name == table_name) {
            table.columns.push(column);
            return true;
        }
        false
    }

    /// 删除指定表的列
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
