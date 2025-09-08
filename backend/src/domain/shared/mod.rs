// 共享的领域类型和值对象

use serde::{Deserialize, Serialize};

/// 数据库连接标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DatabaseId(pub String);

impl DatabaseId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// SQL查询标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueryId(pub String);

impl QueryId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 通用错误类型
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("资源未找到: {resource}")]
    NotFound { resource: String },
    
    #[error("验证失败: {message}")]
    ValidationError { message: String },
    
    #[error("业务规则违反: {rule}")]
    BusinessRuleViolation { rule: String },
    
    #[error("外部依赖错误: {service}")]
    ExternalServiceError { service: String },
    
    #[error("内部错误: {message}")]
    InternalError { message: String },
}

pub type DomainResult<T> = Result<T, DomainError>;

/// 分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

/// 分页结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResult<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl<T> PagedResult<T> {
    pub fn new(items: Vec<T>, total: u64, pagination: &Pagination) -> Self {
        let total_pages = ((total as f64) / (pagination.page_size as f64)).ceil() as u32;
        Self {
            items,
            total,
            page: pagination.page,
            page_size: pagination.page_size,
            total_pages,
        }
    }
}
