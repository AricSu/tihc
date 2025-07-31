use crate::domain::database::DatabaseConnection;
use std::sync::{Arc, Mutex};

pub struct ConnectionStore {
    pub connections: Arc<Mutex<Vec<DatabaseConnection>>>,
}

impl ConnectionStore {
    /// 更新指定 id 的连接信息，返回是否成功
    pub fn update(&self, id: u64, conn: DatabaseConnection) -> bool {
        let mut conns = self.connections.lock().unwrap();
        if let Some(existing) = conns.iter_mut().find(|c| c.id == id) {
            *existing = conn;
            true
        } else {
            false
        }
    }
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, conn: DatabaseConnection) {
        self.connections.lock().unwrap().push(conn);
    }

    pub fn list(&self) -> Vec<DatabaseConnection> {
        self.connections.lock().unwrap().clone()
    }

    pub fn delete(&self, id: u64) -> bool {
        let mut conns = self.connections.lock().unwrap();
        let len_before = conns.len();
        conns.retain(|c| c.id != id);
        len_before != conns.len()
    }
}
