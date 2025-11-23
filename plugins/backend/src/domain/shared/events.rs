// Domain 层：领域事件
// 符合 DDD 原则：领域事件用于解耦聚合根之间的交互

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 领域事件基础 trait
pub trait DomainEvent: Send + Sync {
    /// 事件名称
    fn event_name(&self) -> &str;

    /// 事件发生时间
    fn occurred_at(&self) -> DateTime<Utc>;

    /// 聚合根ID
    fn aggregate_id(&self) -> String;
}

/// 用户相关领域事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserEvent {
    /// 用户已注册
    UserRegistered {
        user_id: i64,
        username: String,
        email: String,
        occurred_at: DateTime<Utc>,
    },

    /// 用户登录成功
    UserLoggedIn {
        user_id: i64,
        username: String,
        login_method: LoginMethod,
        occurred_at: DateTime<Utc>,
    },

    /// 用户信息已更新
    UserProfileUpdated {
        user_id: i64,
        username: String,
        occurred_at: DateTime<Utc>,
    },

    /// GitHub账户已关联
    GitHubAccountLinked {
        user_id: i64,
        github_id: u64,
        github_login: String,
        occurred_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoginMethod {
    Password,
    GitHubOAuth,
}

impl DomainEvent for UserEvent {
    fn event_name(&self) -> &str {
        match self {
            UserEvent::UserRegistered { .. } => "UserRegistered",
            UserEvent::UserLoggedIn { .. } => "UserLoggedIn",
            UserEvent::UserProfileUpdated { .. } => "UserProfileUpdated",
            UserEvent::GitHubAccountLinked { .. } => "GitHubAccountLinked",
        }
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            UserEvent::UserRegistered { occurred_at, .. } => *occurred_at,
            UserEvent::UserLoggedIn { occurred_at, .. } => *occurred_at,
            UserEvent::UserProfileUpdated { occurred_at, .. } => *occurred_at,
            UserEvent::GitHubAccountLinked { occurred_at, .. } => *occurred_at,
        }
    }

    fn aggregate_id(&self) -> String {
        match self {
            UserEvent::UserRegistered { user_id, .. } => user_id.to_string(),
            UserEvent::UserLoggedIn { user_id, .. } => user_id.to_string(),
            UserEvent::UserProfileUpdated { user_id, .. } => user_id.to_string(),
            UserEvent::GitHubAccountLinked { user_id, .. } => user_id.to_string(),
        }
    }
}

/// 领域事件发布器接口
#[async_trait::async_trait]
pub trait DomainEventPublisher: Send + Sync {
    /// 发布领域事件
    async fn publish(&self, event: Box<dyn DomainEvent>)
    -> crate::domain::shared::DomainResult<()>;
}

/// 领域事件处理器接口
#[async_trait::async_trait]
pub trait DomainEventHandler<T: DomainEvent>: Send + Sync {
    /// 处理领域事件
    async fn handle(&self, event: &T) -> crate::domain::shared::DomainResult<()>;
}
