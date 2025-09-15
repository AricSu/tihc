// Editor Domain - SQL编辑器领域模型

use crate::domain::shared::{DatabaseId, DomainError, DomainResult, QueryId};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// SQL查询实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlQuery {
    pub id: QueryId,
    pub database_id: DatabaseId,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: QueryStatus,
}

/// 查询状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryStatus {
    Draft,
    Executing,
    Completed,
    Failed,
    Cancelled,
}

/// 查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub query_id: QueryId,
    pub columns: Vec<String>,
    pub rows: Vec<HashMap<String, serde_json::Value>>,
    pub affected_rows: Option<u64>,
    pub execution_time_ms: u64,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

/// 查询执行上下文
#[derive(Debug, Clone)]
pub struct QueryExecutionContext {
    pub database_id: DatabaseId,
    pub user_id: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub max_rows: Option<u64>,
}

/// SQL查询聚合根
impl SqlQuery {
    pub fn new(database_id: DatabaseId, content: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: QueryId::new(Uuid::new_v4().to_string()),
            database_id,
            content,
            created_at: now,
            updated_at: now,
            status: QueryStatus::Draft,
        }
    }

    pub fn update_content(&mut self, content: String) -> DomainResult<()> {
        if content.trim().is_empty() {
            return Err(DomainError::ValidationError {
                message: "SQL内容不能为空".to_string(),
            });
        }

        self.content = content;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn start_execution(&mut self) -> DomainResult<()> {
        match self.status {
            QueryStatus::Draft | QueryStatus::Failed | QueryStatus::Cancelled => {
                self.status = QueryStatus::Executing;
                self.updated_at = chrono::Utc::now();
                Ok(())
            }
            _ => Err(DomainError::BusinessRuleViolation {
                rule: "只有草稿、失败或取消状态的查询才能执行".to_string(),
            }),
        }
    }

    pub fn complete_execution(&mut self) -> DomainResult<()> {
        if self.status != QueryStatus::Executing {
            return Err(DomainError::BusinessRuleViolation {
                rule: "只有正在执行的查询才能标记为完成".to_string(),
            });
        }

        self.status = QueryStatus::Completed;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn fail_execution(&mut self, _error: &str) -> DomainResult<()> {
        if self.status != QueryStatus::Executing {
            return Err(DomainError::BusinessRuleViolation {
                rule: "只有正在执行的查询才能标记为失败".to_string(),
            });
        }

        self.status = QueryStatus::Failed;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }
}

/// SQL查询仓储接口
#[async_trait]
pub trait SqlQueryRepository {
    async fn save(&self, query: &SqlQuery) -> DomainResult<()>;
    async fn find_by_id(&self, id: &QueryId) -> DomainResult<Option<SqlQuery>>;
    async fn find_by_database(&self, database_id: &DatabaseId) -> DomainResult<Vec<SqlQuery>>;
    async fn delete(&self, id: &QueryId) -> DomainResult<()>;
}

/// SQL执行器接口
#[async_trait]
pub trait SqlExecutor {
    async fn execute_query(
        &self,
        query: &SqlQuery,
        context: &QueryExecutionContext,
    ) -> DomainResult<QueryResult>;

    async fn validate_syntax(&self, sql: &str) -> DomainResult<bool>;
}

/// 编辑器领域服务
pub struct EditorDomainService {
    query_repository: Box<dyn SqlQueryRepository + Send + Sync>,
    sql_executor: Box<dyn SqlExecutor + Send + Sync>,
}

impl EditorDomainService {
    pub fn new(
        query_repository: Box<dyn SqlQueryRepository + Send + Sync>,
        sql_executor: Box<dyn SqlExecutor + Send + Sync>,
    ) -> Self {
        Self {
            query_repository,
            sql_executor,
        }
    }

    pub async fn execute_query(
        &self,
        query_id: &QueryId,
        context: QueryExecutionContext,
    ) -> DomainResult<QueryResult> {
        // 获取查询
        let mut query = self
            .query_repository
            .find_by_id(query_id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: format!("查询 {}", query_id.as_str()),
            })?;

        // 开始执行
        query.start_execution()?;
        self.query_repository.save(&query).await?;

        // 执行查询
        match self.sql_executor.execute_query(&query, &context).await {
            Ok(result) => {
                query.complete_execution()?;
                self.query_repository.save(&query).await?;
                Ok(result)
            }
            Err(error) => {
                query.fail_execution(&error.to_string())?;
                self.query_repository.save(&query).await?;
                Err(error)
            }
        }
    }
}
