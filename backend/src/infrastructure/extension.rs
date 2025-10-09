// Extension Infrastructure Layer
// 浏览器扩展数据存储基础设施实现

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::domain::extension::{AuthInfo, CollectionRequest, ExtensionRepository};

/// 内存中的扩展数据存储库实现
#[derive(Debug)]
pub struct InMemoryExtensionRepository {
    /// 存储收集的原始数据，按照域名分组
    collected_data: Arc<RwLock<HashMap<String, Vec<CollectionRequest>>>>,
    /// 存储提取的认证信息，按照域名索引
    auth_info: Arc<RwLock<HashMap<String, AuthInfo>>>,
}

impl InMemoryExtensionRepository {
    /// 创建新的内存存储库实例
    pub fn new() -> Self {
        Self {
            collected_data: Arc::new(RwLock::new(HashMap::new())),
            auth_info: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 获取存储的原始数据统计
    pub fn get_stats(&self) -> Result<ExtensionRepositoryStats, String> {
        let collected_data = self
            .collected_data
            .read()
            .map_err(|e| format!("Failed to read collected data: {}", e))?;
        let auth_info = self
            .auth_info
            .read()
            .map_err(|e| format!("Failed to read auth info: {}", e))?;

        let total_collections = collected_data.values().map(|v| v.len()).sum();
        let domains_count = collected_data.len();
        let auth_info_count = auth_info.len();

        Ok(ExtensionRepositoryStats {
            total_collections,
            domains_count,
            auth_info_count,
        })
    }

    /// 获取特定域名的收集历史
    pub fn get_domain_collection_history(
        &self,
        domain: &str,
    ) -> Result<Vec<CollectionRequest>, String> {
        let collected_data = self
            .collected_data
            .read()
            .map_err(|e| format!("Failed to read collected data: {}", e))?;

        Ok(collected_data.get(domain).cloned().unwrap_or_default())
    }
}

impl Default for InMemoryExtensionRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtensionRepository for InMemoryExtensionRepository {
    fn store_collected_data(&self, request: &CollectionRequest) -> Result<(), String> {
        // 存储原始数据
        {
            let mut collected_data = self
                .collected_data
                .write()
                .map_err(|e| format!("Failed to write collected data: {}", e))?;

            collected_data
                .entry(request.domain.clone())
                .or_insert_with(Vec::new)
                .push(request.clone());
        }

        // 提取并存储认证信息
        let auth_info = request.extract_auth_info();
        {
            let mut auth_storage = self
                .auth_info
                .write()
                .map_err(|e| format!("Failed to write auth info: {}", e))?;

            // 如果已有该域名的认证信息，则合并新的信息
            if let Some(existing_auth) = auth_storage.get_mut(&request.domain) {
                // 合并 tokens，新的覆盖旧的
                existing_auth.tokens.extend(auth_info.tokens);

                // 合并 cookies，避免重复
                for new_cookie in auth_info.cookies {
                    if !existing_auth
                        .cookies
                        .iter()
                        .any(|c| c.name == new_cookie.name)
                    {
                        existing_auth.cookies.push(new_cookie);
                    }
                }

                // 更新时间戳为最新的
                if auth_info.timestamp > existing_auth.timestamp {
                    existing_auth.timestamp = auth_info.timestamp;
                }

                // 更新页面类型（如果有更具体的信息）
                if auth_info.page_type != crate::domain::extension::PageType::Unknown {
                    existing_auth.page_type = auth_info.page_type;
                }
            } else {
                auth_storage.insert(request.domain.clone(), auth_info);
            }
        }

        tracing::debug!("Stored collection data for domain: {}", request.domain);
        Ok(())
    }

    fn get_auth_info(&self, domain: &str) -> Result<Option<AuthInfo>, String> {
        let auth_info = self
            .auth_info
            .read()
            .map_err(|e| format!("Failed to read auth info: {}", e))?;

        Ok(auth_info.get(domain).cloned())
    }

    fn get_all_auth_info(&self) -> Result<Vec<AuthInfo>, String> {
        let auth_info = self
            .auth_info
            .read()
            .map_err(|e| format!("Failed to read auth info: {}", e))?;

        Ok(auth_info.values().cloned().collect())
    }

    fn clear_domain_data(&self, domain: &str) -> Result<(), String> {
        {
            let mut collected_data = self
                .collected_data
                .write()
                .map_err(|e| format!("Failed to write collected data: {}", e))?;
            collected_data.remove(domain);
        }

        {
            let mut auth_info = self
                .auth_info
                .write()
                .map_err(|e| format!("Failed to write auth info: {}", e))?;
            auth_info.remove(domain);
        }

        tracing::info!("Cleared all data for domain: {}", domain);
        Ok(())
    }

    fn clear_all_data(&self) -> Result<(), String> {
        {
            let mut collected_data = self
                .collected_data
                .write()
                .map_err(|e| format!("Failed to write collected data: {}", e))?;
            collected_data.clear();
        }

        {
            let mut auth_info = self
                .auth_info
                .write()
                .map_err(|e| format!("Failed to write auth info: {}", e))?;
            auth_info.clear();
        }

        tracing::info!("Cleared all extension data");
        Ok(())
    }
}

/// 扩展存储库统计信息
#[derive(Debug, Clone)]
pub struct ExtensionRepositoryStats {
    pub total_collections: usize,
    pub domains_count: usize,
    pub auth_info_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::extension::{CollectedData, PageType};
    use std::collections::HashMap;

    fn create_test_request(domain: &str, page_type: &str) -> CollectionRequest {
        let mut local_storage = HashMap::new();
        local_storage.insert("test_token".to_string(), "token123".to_string());

        CollectionRequest {
            url: format!("https://{}", domain),
            domain: domain.to_string(),
            timestamp: 1234567890,
            task_id: Some("test-task".to_string()),
            data: CollectedData {
                cookies: "session=session123; auth=auth456".to_string(),
                page_type: page_type.to_string(),
                user_agent: "Mozilla/5.0".to_string(),
                local_storage: Some(local_storage),
                session_storage: None,
            },
        }
    }

    #[test]
    fn test_store_and_retrieve_auth_info() {
        let repo = InMemoryExtensionRepository::new();
        let request = create_test_request("example.com", "grafana");

        // Store data
        assert!(repo.store_collected_data(&request).is_ok());

        // Retrieve auth info
        let auth_info = repo.get_auth_info("example.com").unwrap();
        assert!(auth_info.is_some());

        let auth_info = auth_info.unwrap();
        assert_eq!(auth_info.domain, "example.com");
        assert_eq!(auth_info.page_type, PageType::Grafana);
        assert!(!auth_info.tokens.is_empty());
        assert!(!auth_info.cookies.is_empty());
    }

    #[test]
    fn test_merge_auth_info() {
        let repo = InMemoryExtensionRepository::new();

        // First request
        let request1 = create_test_request("example.com", "grafana");
        assert!(repo.store_collected_data(&request1).is_ok());

        // Second request with different tokens
        let mut local_storage2 = HashMap::new();
        local_storage2.insert("another_token".to_string(), "token456".to_string());

        let request2 = CollectionRequest {
            url: "https://example.com".to_string(),
            domain: "example.com".to_string(),
            timestamp: 1234567900, // Later timestamp
            task_id: Some("test-task-2".to_string()),
            data: CollectedData {
                cookies: "new_session=new_session123".to_string(),
                page_type: "grafana".to_string(),
                user_agent: "Mozilla/5.0".to_string(),
                local_storage: Some(local_storage2),
                session_storage: None,
            },
        };

        assert!(repo.store_collected_data(&request2).is_ok());

        // Check merged auth info
        let auth_info = repo.get_auth_info("example.com").unwrap().unwrap();
        assert!(auth_info.tokens.len() >= 2); // Should have merged tokens
        assert_eq!(auth_info.timestamp, 1234567900); // Should have updated timestamp
    }

    #[test]
    fn test_clear_domain_data() {
        let repo = InMemoryExtensionRepository::new();
        let request = create_test_request("example.com", "clinic");

        assert!(repo.store_collected_data(&request).is_ok());
        assert!(repo.get_auth_info("example.com").unwrap().is_some());

        assert!(repo.clear_domain_data("example.com").is_ok());
        assert!(repo.get_auth_info("example.com").unwrap().is_none());
    }

    #[test]
    fn test_clear_all_data() {
        let repo = InMemoryExtensionRepository::new();

        let request1 = create_test_request("example1.com", "grafana");
        let request2 = create_test_request("example2.com", "clinic");

        assert!(repo.store_collected_data(&request1).is_ok());
        assert!(repo.store_collected_data(&request2).is_ok());

        let all_auth = repo.get_all_auth_info().unwrap();
        assert_eq!(all_auth.len(), 2);

        assert!(repo.clear_all_data().is_ok());

        let all_auth_after_clear = repo.get_all_auth_info().unwrap();
        assert_eq!(all_auth_after_clear.len(), 0);
    }

    #[test]
    fn test_get_stats() {
        let repo = InMemoryExtensionRepository::new();

        let request1 = create_test_request("example1.com", "grafana");
        let request2 = create_test_request("example2.com", "clinic");
        let request3 = create_test_request("example1.com", "grafana"); // Same domain

        assert!(repo.store_collected_data(&request1).is_ok());
        assert!(repo.store_collected_data(&request2).is_ok());
        assert!(repo.store_collected_data(&request3).is_ok());

        let stats = repo.get_stats().unwrap();
        assert_eq!(stats.total_collections, 3);
        assert_eq!(stats.domains_count, 2);
        assert_eq!(stats.auth_info_count, 2);
    }
}
