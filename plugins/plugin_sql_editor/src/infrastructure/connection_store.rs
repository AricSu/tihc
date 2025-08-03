use crate::domain::database::DatabaseConnection;
use crate::domain::database::DatabasePool;
use crate::domain::error::SqlEditorError;
use crate::infrastructure::error::MutexExt;
use crate::infrastructure::error::StoreError;
use async_trait::async_trait;
use sqlx::mysql::MySqlPool;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait ConnectionOps: Send + Sync {
    async fn add_connection(&self, conn: DatabaseConnection) -> Result<(), SqlEditorError>;
    async fn list_connection(&self) -> Result<Vec<DatabaseConnection>, SqlEditorError>;
    async fn delete_connection(&self, id: u64) -> Result<bool, SqlEditorError>;
    async fn update_connection(
        &self,
        id: u64,
        update: DatabaseConnection,
    ) -> Result<bool, SqlEditorError>;
    async fn get_connection(&self, id: u64) -> Result<Option<DatabaseConnection>, SqlEditorError>;
    async fn test_connection(&self, conn: &DatabaseConnection) -> Result<bool, SqlEditorError>;
}

/// 连接存储，负责管理所有数据库连接，线程安全
pub struct ConnectionStore {
    pub connections: Arc<Mutex<Vec<DatabaseConnection>>>,
}

impl ConnectionStore {
    /// 新建空存储
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 添加连接
    pub fn add(&self, mut conn: DatabaseConnection) -> Result<(), StoreError> {
        tracing::info!(target: "connection_store", "add: id={}, engine={}, host={}, port={}, db={:?}", conn.id, conn.engine, conn.host, conn.port, conn.database);
        if conn.engine == "mysql" || conn.engine == "tidb" {
            let pool_url = format!(
                "mysql://{}:{}@{}:{}/{}",
                conn.username,
                conn.password.as_deref().unwrap_or("") ,
                conn.host,
                conn.port,
                conn.database.as_deref().unwrap_or("")
            );
            tracing::info!(target: "connection_store", "add: try create mysql pool, url={}", pool_url);
            let pool = MySqlPool::connect_lazy(&pool_url).ok();
            if pool.is_some() {
                tracing::info!(target: "connection_store", "add: mysql pool created");
            } else {
                tracing::warn!(target: "connection_store", "add: mysql pool create failed");
            }
            conn.pool = pool.map(|p| DatabasePool::MySql(Arc::new(p)));
        } else {
            tracing::info!(target: "connection_store", "add: engine not implemented, id={}, engine={}", conn.id, conn.engine);
        }
        try_lock!(self.connections).push(conn);
        Ok(())
    }

    pub fn get_pool(&self, id: u64) -> Option<DatabasePool> {
        let conns = self.connections.lock_safe().ok()?;
        let conn = conns.iter().find(|c| c.id == id);
        if let Some(c) = conn {
            tracing::info!(target: "connection_store", "get_pool: id={}, engine={}, pool={:?}", c.id, c.engine, c.pool.is_some());
            c.pool.clone()
        } else {
            tracing::warn!(target: "connection_store", "get_pool: id={} not found", id);
            None
        }
    }

    /// 获取所有连接
    pub fn list(&self) -> Result<Vec<DatabaseConnection>, StoreError> {
        Ok(try_lock!(self.connections).clone())
    }

    /// 根据 id 获取连接
    pub fn get(&self, id: u64) -> Result<Option<DatabaseConnection>, StoreError> {
        let conns = try_lock!(self.connections);
        let conn = conns.iter().find(|c| c.id == id).cloned();
        if let Some(ref c) = conn {
            tracing::info!(target: "connection_store", "get: id={}, engine={}", c.id, c.engine);
        } else {
            tracing::warn!(target: "connection_store", "get: id={} not found", id);
        }
        Ok(conn)
    }

    /// 更新指定 id 的连接
    pub fn update(&self, id: u64, conn: DatabaseConnection) -> Result<bool, StoreError> {
        let mut conns = try_lock!(self.connections);
        if let Some(existing) = conns.iter_mut().find(|c| c.id == id) {
            *existing = conn;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 删除指定 id 的连接
    pub fn delete(&self, id: u64) -> Result<bool, StoreError> {
        let mut conns = try_lock!(self.connections);
        let len_before = conns.len();
        conns.retain(|c| c.id != id);
        Ok(len_before != conns.len())
    }
}

#[async_trait]
impl ConnectionOps for ConnectionStore {
    async fn add_connection(&self, conn: DatabaseConnection) -> Result<(), SqlEditorError> {
        self.add(conn).map_err(SqlEditorError::Infra)
    }
    async fn list_connection(&self) -> Result<Vec<DatabaseConnection>, SqlEditorError> {
        self.list().map_err(SqlEditorError::Infra)
    }
    async fn delete_connection(&self, id: u64) -> Result<bool, SqlEditorError> {
        self.delete(id).map_err(SqlEditorError::Infra)
    }
    async fn update_connection(
        &self,
        id: u64,
        update: DatabaseConnection,
    ) -> Result<bool, SqlEditorError> {
        self.update(id, update).map_err(SqlEditorError::Infra)
    }
    async fn get_connection(&self, id: u64) -> Result<Option<DatabaseConnection>, SqlEditorError> {
        self.get(id).map_err(SqlEditorError::Infra)
    }
    async fn test_connection(&self, conn: &DatabaseConnection) -> Result<bool, SqlEditorError> {
        // 实际可实现连接测试逻辑
        Ok(conn.engine == "mysql" || conn.engine == "tidb")
    }
}