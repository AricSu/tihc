use crate::domain::database::DatabaseConnection;
use crate::infrastructure::error::*;
use std::sync::{Arc, Mutex};
// use super::error::{StoreError, MutexExt};

pub struct ConnectionStore {
    pub connections: Arc<Mutex<Vec<DatabaseConnection>>>,
}

impl ConnectionStore {
    /// 根据 id 获取连接
    /// Get a connection by id. Returns None if not found, or error if lock fails.
    pub fn get(&self, id: u64) -> Result<Option<DatabaseConnection>, StoreError> {
        let conns = try_lock!(self.connections);
        Ok(conns.iter().find(|c| c.id == id).cloned())
    }
    /// 更新指定 id 的连接信息，返回是否成功
    /// Update a connection by id. Returns true if updated, false if not found, or error if lock fails.
    pub fn update(&self, id: u64, conn: DatabaseConnection) -> Result<bool, StoreError> {
        let mut conns = try_lock!(self.connections);
        if let Some(existing) = conns.iter_mut().find(|c| c.id == id) {
            *existing = conn;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a new connection. Returns error if lock fails.
    pub fn add(&self, conn: DatabaseConnection) -> Result<(), StoreError> {
        try_lock!(self.connections).push(conn);
        Ok(())
    }

    /// List all connections. Returns error if lock fails.
    pub fn list(&self) -> Result<Vec<DatabaseConnection>, StoreError> {
        Ok(try_lock!(self.connections).clone())
    }

    /// Delete a connection by id. Returns true if deleted, false if not found, or error if lock fails.
    pub fn delete(&self, id: u64) -> Result<bool, StoreError> {
        let mut conns = try_lock!(self.connections);
        let len_before = conns.len();
        conns.retain(|c| c.id != id);
        Ok(len_before != conns.len())
    }
}
