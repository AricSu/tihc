// Extension Application Service
// 浏览器扩展数据收集的应用服务层

use std::sync::Arc;

use crate::domain::extension::{
    AuthInfo, CollectionRequest, CollectionResponse, ExtensionRepository,
};

/// 扩展应用服务接口
#[async_trait::async_trait]
pub trait ExtensionApplicationService: Send + Sync {
    /// 处理数据收集请求
    async fn handle_collection(&self, request: CollectionRequest) -> Result<CollectionResponse, String>;
    
    /// 获取特定域名的认证信息
    async fn get_auth_info(&self, domain: &str) -> Result<Option<AuthInfo>, String>;
    
    /// 获取所有存储的认证信息
    async fn get_all_auth_info(&self) -> Result<Vec<AuthInfo>, String>;
    
    /// 清除特定域名的数据
    async fn clear_domain_data(&self, domain: &str) -> Result<(), String>;
    
    /// 清除所有数据
    async fn clear_all_data(&self) -> Result<(), String>;
}

/// 扩展应用服务实现
pub struct ExtensionApplicationServiceImpl {
    repository: Arc<dyn ExtensionRepository>,
}

impl ExtensionApplicationServiceImpl {
    pub fn new(repository: Arc<dyn ExtensionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ExtensionApplicationService for ExtensionApplicationServiceImpl {
    async fn handle_collection(&self, request: CollectionRequest) -> Result<CollectionResponse, String> {
        // 1. 验证请求
        if let Err(error) = request.validate() {
            return Ok(CollectionResponse::error(
                format!("Invalid request: {}", error),
                request.task_id.clone(),
            ));
        }
        
        // 2. 记录收集请求
        tracing::info!(
            "Processing collection request for domain: {}, page_type: {}, task_id: {:?}",
            request.domain,
            request.data.page_type,
            request.task_id
        );
        
        // 3. 存储原始数据
        if let Err(error) = self.repository.store_collected_data(&request) {
            tracing::error!("Failed to store collected data: {}", error);
            return Ok(CollectionResponse::error(
                format!("Failed to store data: {}", error),
                request.task_id.clone(),
            ));
        }
        
        // 4. 提取并存储认证信息
        let auth_info = request.extract_auth_info();
        tracing::info!(
            "Extracted {} tokens and {} cookies for domain {}",
            auth_info.tokens.len(),
            auth_info.cookies.len(),
            auth_info.domain
        );
        
        // 打印详细的token和cookie信息
        if !auth_info.tokens.is_empty() {
            tracing::info!("Extracted tokens for {}: {:?}", auth_info.domain, auth_info.tokens);
        }
        if !auth_info.cookies.is_empty() {
            tracing::info!("Extracted cookies for {}: {:?}", auth_info.domain, auth_info.cookies);
        }
        
        // 5. 记录成功
        tracing::info!(
            "Successfully processed collection request for domain: {}",
            request.domain
        );
        
        Ok(CollectionResponse::success(
            format!(
                "Successfully collected data for domain: {} (page_type: {})",
                request.domain, request.data.page_type
            ),
            request.task_id.clone(),
        ))
    }
    
    async fn get_auth_info(&self, domain: &str) -> Result<Option<AuthInfo>, String> {
        tracing::debug!("Getting auth info for domain: {}", domain);
        self.repository.get_auth_info(domain)
    }
    
    async fn get_all_auth_info(&self) -> Result<Vec<AuthInfo>, String> {
        tracing::debug!("Getting all auth info");
        self.repository.get_all_auth_info()
    }
    
    async fn clear_domain_data(&self, domain: &str) -> Result<(), String> {
        tracing::info!("Clearing data for domain: {}", domain);
        self.repository.clear_domain_data(domain)
    }
    
    async fn clear_all_data(&self) -> Result<(), String> {
        tracing::info!("Clearing all extension data");
        self.repository.clear_all_data()
    }
}

/// 创建默认的扩展应用服务
pub fn create_extension_service() -> Arc<dyn ExtensionApplicationService> {
    let repository = Arc::new(crate::infrastructure::extension::InMemoryExtensionRepository::new());
    Arc::new(ExtensionApplicationServiceImpl::new(repository))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::extension::CollectedData;
    use std::collections::HashMap;
    
    struct MockExtensionRepository {
        should_fail: bool,
    }
    
    impl MockExtensionRepository {
        fn new(should_fail: bool) -> Self {
            Self { should_fail }
        }
    }
    
    impl ExtensionRepository for MockExtensionRepository {
        fn store_collected_data(&self, _request: &CollectionRequest) -> Result<(), String> {
            if self.should_fail {
                Err("Storage failed".to_string())
            } else {
                Ok(())
            }
        }
        
        fn get_auth_info(&self, _domain: &str) -> Result<Option<AuthInfo>, String> {
            Ok(None)
        }
        
        fn get_all_auth_info(&self) -> Result<Vec<AuthInfo>, String> {
            Ok(vec![])
        }
        
        fn clear_domain_data(&self, _domain: &str) -> Result<(), String> {
            Ok(())
        }
        
        fn clear_all_data(&self) -> Result<(), String> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_handle_collection_success() {
        let repository = Arc::new(MockExtensionRepository::new(false));
        let service = ExtensionApplicationServiceImpl::new(repository);
        
        let request = CollectionRequest {
            url: "https://grafana.example.com".to_string(),
            domain: "grafana.example.com".to_string(),
            timestamp: 1234567890,
            task_id: Some("test-task".to_string()),
            data: CollectedData {
                cookies: "session=abc123".to_string(),
                page_type: "grafana".to_string(),
                user_agent: "Mozilla/5.0".to_string(),
                local_storage: Some(HashMap::new()),
                session_storage: None,
            },
        };
        
        let result = service.handle_collection(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert!(response.message.contains("grafana.example.com"));
    }
    
    #[tokio::test]
    async fn test_handle_collection_validation_failure() {
        let repository = Arc::new(MockExtensionRepository::new(false));
        let service = ExtensionApplicationServiceImpl::new(repository);
        
        let invalid_request = CollectionRequest {
            url: "".to_string(), // Empty URL should fail validation
            domain: "example.com".to_string(),
            timestamp: 1234567890,
            task_id: None,
            data: CollectedData {
                cookies: "".to_string(),
                page_type: "unknown".to_string(),
                user_agent: "".to_string(),
                local_storage: None,
                session_storage: None,
            },
        };
        
        let result = service.handle_collection(invalid_request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(!response.success);
        assert!(response.message.contains("Invalid request"));
    }
    
    #[tokio::test]
    async fn test_handle_collection_storage_failure() {
        let repository = Arc::new(MockExtensionRepository::new(true));
        let service = ExtensionApplicationServiceImpl::new(repository);
        
        let request = CollectionRequest {
            url: "https://example.com".to_string(),
            domain: "example.com".to_string(),
            timestamp: 1234567890,
            task_id: None,
            data: CollectedData {
                cookies: "session=abc123".to_string(),
                page_type: "unknown".to_string(),
                user_agent: "Mozilla/5.0".to_string(),
                local_storage: None,
                session_storage: None,
            },
        };
        
        let result = service.handle_collection(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(!response.success);
        assert!(response.message.contains("Failed to store data"));
    }
}