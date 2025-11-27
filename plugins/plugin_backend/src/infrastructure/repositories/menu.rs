use serde_json::Value;
use sqlx::MySqlPool;

use crate::domain::shared::{DomainError, DomainResult};

/// MySQL 菜单/菜单树仓储实现（存储 JSON blob）
pub struct MySqlMenuRepository {
    pool: MySqlPool,
}

impl MySqlMenuRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 根据 name 返回 JSON 菜单树（如果存在）
    pub async fn get_menu_tree(&self, name: &str) -> DomainResult<Option<Value>> {
        let row = sqlx::query_scalar::<_, Value>("SELECT data FROM tihc_menu_tree WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::InternalError {
                message: format!("Database error: {}", e),
            })?;

        Ok(row)
    }
}
