use crate::domain::shared::DomainError;

/// 认证错误映射服务：将领域错误映射为应用层错误代码
pub struct AuthErrorMapper;

impl AuthErrorMapper {
    pub fn map_error_code(error: &DomainError) -> u32 {
        match error {
            DomainError::ValidationError { .. } => 10003,
            DomainError::NotFound { .. } => 10001,
            DomainError::BusinessRuleViolation { .. } => 10002,
            DomainError::InternalError { .. } => 10000,
            DomainError::AuthenticationError { .. } => 10005,
        }
    }

    pub fn get_user_message(error: &DomainError) -> String {
        match error {
            DomainError::ValidationError { message } => message.clone(),
            DomainError::NotFound { resource } => format!("{}不存在", resource),
            DomainError::BusinessRuleViolation { rule } => format!("业务规则违反: {}", rule),
            DomainError::InternalError { .. } => "系统内部错误".to_string(),
            DomainError::AuthenticationError { message } => message.clone(),
        }
    }
}
