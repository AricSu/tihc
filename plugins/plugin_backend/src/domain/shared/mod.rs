pub mod services;

/// 通用错误类型
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("资源未找到: {resource}")]
    NotFound { resource: String },

    #[error("验证失败: {message}")]
    ValidationError { message: String },

    #[error("业务规则违反: {rule}")]
    BusinessRuleViolation { rule: String },

    #[error("内部错误: {message}")]
    InternalError { message: String },

    #[error("认证失败: {message}")]
    AuthenticationError { message: String },
}

pub type DomainResult<T> = Result<T, DomainError>;

// 为 DomainError 实现 anyhow::Error 转换
impl From<anyhow::Error> for DomainError {
    fn from(err: anyhow::Error) -> Self {
        DomainError::InternalError {
            message: err.to_string(),
        }
    }
}
