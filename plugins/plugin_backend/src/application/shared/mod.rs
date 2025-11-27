// Application 层：应用服务共享组件
// 符合 DDD 原则：应用层负责编排业务用例，处理事务和领域事件

use crate::domain::shared::{DomainEvent, DomainEventPublisher, DomainResult};
use std::sync::Arc;

/// 应用服务基础接口
#[async_trait::async_trait]
pub trait ApplicationService: Send + Sync {
    /// 获取服务名称
    fn service_name(&self) -> &str;
}

/// 应用服务基类
pub struct BaseApplicationService {
    event_publisher: Option<Arc<dyn DomainEventPublisher>>,
    service_name: String,
}

impl BaseApplicationService {
    pub fn new(
        service_name: String,
        event_publisher: Option<Arc<dyn DomainEventPublisher>>,
    ) -> Self {
        Self {
            event_publisher,
            service_name,
        }
    }

    /// 发布领域事件
    pub async fn publish_event(&self, event: Box<dyn DomainEvent>) -> DomainResult<()> {
        if let Some(publisher) = &self.event_publisher {
            publisher.publish(event).await?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl ApplicationService for BaseApplicationService {
    fn service_name(&self) -> &str {
        &self.service_name
    }
}

// TODO: 这些类型应该移动到 domain/shared 模块中，这里暂时保留避免编译错误
// 这违反了 DDD 原则，应该在后续重构中修复
