// Editor Application Service Implementation

use crate::application::services::*;
use crate::domain::{
    editor::{SqlQuery, SqlQueryRepository, SqlExecutor, QueryExecutionContext, EditorDomainService},
    shared::{DomainResult, DatabaseId, QueryId, Pagination, PagedResult},
};
use async_trait::async_trait;

pub struct EditorApplicationServiceImpl {
    domain_service: EditorDomainService,
    query_repository: Box<dyn SqlQueryRepository + Send + Sync>,
}

impl EditorApplicationServiceImpl {
    pub fn new(
        domain_service: EditorDomainService,
        query_repository: Box<dyn SqlQueryRepository + Send + Sync>,
    ) -> Self {
        Self {
            domain_service,
            query_repository,
        }
    }
}

#[async_trait]
impl EditorApplicationService for EditorApplicationServiceImpl {
    async fn create_query(&self, request: CreateQueryRequest) -> DomainResult<QueryResponse> {
        let database_id = DatabaseId::new(request.database_id);
        let mut query = SqlQuery::new(database_id, request.content);
        
        self.query_repository.save(&query).await?;
        
        Ok(QueryResponse {
            id: query.id.as_str().to_string(),
            database_id: query.database_id.as_str().to_string(),
            content: query.content,
            status: format!("{:?}", query.status),
            created_at: query.created_at,
            updated_at: query.updated_at,
        })
    }
    
    async fn execute_query(&self, request: ExecuteQueryRequest) -> DomainResult<QueryResultResponse> {
        let query_id = QueryId::new(request.query_id);
        let context = QueryExecutionContext {
            database_id: DatabaseId::new(""), // 将从查询中获取
            user_id: None, // TODO: 从认证上下文获取
            timeout_seconds: request.timeout_seconds,
            max_rows: request.max_rows,
        };
        
        let result = self.domain_service.execute_query(&query_id, context).await?;
        
        Ok(QueryResultResponse {
            query_id: result.query_id.as_str().to_string(),
            columns: result.columns,
            rows: result.rows,
            affected_rows: result.affected_rows,
            execution_time_ms: result.execution_time_ms,
            executed_at: result.executed_at,
        })
    }
    
    async fn get_query(&self, query_id: &str) -> DomainResult<QueryResponse> {
        let query_id = QueryId::new(query_id);
        let query = self
            .query_repository
            .find_by_id(&query_id)
            .await?
            .ok_or_else(|| crate::domain::shared::DomainError::NotFound {
                resource: format!("查询 {}", query_id.as_str()),
            })?;
        
        Ok(QueryResponse {
            id: query.id.as_str().to_string(),
            database_id: query.database_id.as_str().to_string(),
            content: query.content,
            status: format!("{:?}", query.status),
            created_at: query.created_at,
            updated_at: query.updated_at,
        })
    }
    
    async fn list_queries(
        &self,
        database_id: &str,
        pagination: Pagination,
    ) -> DomainResult<PagedResult<QueryResponse>> {
        let database_id = DatabaseId::new(database_id);
        let queries = self.query_repository.find_by_database(&database_id).await?;
        
        // 简单的内存分页实现（生产环境应该在数据库层面实现）
        let total = queries.len() as u64;
        let start = ((pagination.page - 1) * pagination.page_size) as usize;
        let end = (start + pagination.page_size as usize).min(queries.len());
        
        let page_queries: Vec<QueryResponse> = queries[start..end]
            .iter()
            .map(|query| QueryResponse {
                id: query.id.as_str().to_string(),
                database_id: query.database_id.as_str().to_string(),
                content: query.content.clone(),
                status: format!("{:?}", query.status),
                created_at: query.created_at,
                updated_at: query.updated_at,
            })
            .collect();
        
        Ok(PagedResult::new(page_queries, total, &pagination))
    }
    
    async fn update_query(&self, request: UpdateQueryRequest) -> DomainResult<QueryResponse> {
        let query_id = QueryId::new(request.query_id);
        let mut query = self
            .query_repository
            .find_by_id(&query_id)
            .await?
            .ok_or_else(|| crate::domain::shared::DomainError::NotFound {
                resource: format!("查询 {}", query_id.as_str()),
            })?;
        
        query.update_content(request.content)?;
        self.query_repository.save(&query).await?;
        
        Ok(QueryResponse {
            id: query.id.as_str().to_string(),
            database_id: query.database_id.as_str().to_string(),
            content: query.content,
            status: format!("{:?}", query.status),
            created_at: query.created_at,
            updated_at: query.updated_at,
        })
    }
    
    async fn delete_query(&self, query_id: &str) -> DomainResult<()> {
        let query_id = QueryId::new(query_id);
        self.query_repository.delete(&query_id).await
    }
}
