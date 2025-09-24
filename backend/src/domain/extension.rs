// Extension Domain Layer
// 浏览器扩展数据收集相关的领域模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 数据收集请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRequest {
    pub url: String,
    pub domain: String,
    pub timestamp: i64,
    pub task_id: Option<String>,
    pub data: CollectedData,
}

/// 收集到的数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedData {
    pub cookies: String,
    pub page_type: String,
    pub user_agent: String,
    pub local_storage: Option<HashMap<String, String>>,
    pub session_storage: Option<HashMap<String, String>>,
}

/// 数据收集响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResponse {
    pub success: bool,
    pub message: String,
    pub task_id: Option<String>,
    pub timestamp: i64,
}

/// 存储的认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    pub domain: String,
    pub page_type: PageType,
    pub tokens: HashMap<String, String>,
    pub cookies: Vec<Cookie>,
    pub timestamp: i64,
}

/// 页面类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageType {
    Grafana,
    Clinic,
    Unknown,
}

impl From<String> for PageType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "grafana" => PageType::Grafana,
            "clinic" => PageType::Clinic,
            _ => PageType::Unknown,
        }
    }
}

/// Cookie 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
}

/// 数据收集存储库接口
pub trait ExtensionRepository: Send + Sync {
    /// 存储收集的数据
    fn store_collected_data(&self, request: &CollectionRequest) -> Result<(), String>;
    
    /// 获取特定域名的认证信息
    fn get_auth_info(&self, domain: &str) -> Result<Option<AuthInfo>, String>;
    
    /// 获取所有存储的认证信息
    fn get_all_auth_info(&self) -> Result<Vec<AuthInfo>, String>;
    
    /// 清除特定域名的数据
    fn clear_domain_data(&self, domain: &str) -> Result<(), String>;
    
    /// 清除所有数据
    fn clear_all_data(&self) -> Result<(), String>;
}

impl CollectionRequest {
    /// 验证请求数据的有效性
    pub fn validate(&self) -> Result<(), String> {
        if self.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        
        if self.domain.is_empty() {
            return Err("Domain cannot be empty".to_string());
        }
        
        if self.timestamp <= 0 {
            return Err("Timestamp must be positive".to_string());
        }
        
        Ok(())
    }
    
    /// 提取认证信息
    pub fn extract_auth_info(&self) -> AuthInfo {
        let page_type = PageType::from(self.data.page_type.clone());
        let mut tokens = HashMap::new();
        let mut cookies = Vec::new();
        
        // 临时调试：打印原始的localStorage和cookies数据
        tracing::info!("Raw cookies string: {}", self.data.cookies);
        if let Some(ref local_storage) = self.data.local_storage {
            tracing::info!("Raw localStorage: {:?}", local_storage);
        }
        if let Some(ref session_storage) = self.data.session_storage {
            tracing::info!("Raw sessionStorage: {:?}", session_storage);
        }
        
        // 解析 cookies 字符串
        if !self.data.cookies.is_empty() {
            for cookie_str in self.data.cookies.split(';') {
                let cookie_str = cookie_str.trim();
                if let Some((name, value)) = cookie_str.split_once('=') {
                    let cookie = Cookie {
                        name: name.trim().to_string(),
                        value: value.trim().to_string(),
                        domain: Some(self.domain.clone()),
                        path: None,
                    };
                    cookies.push(cookie);
                }
            }
        }
        
        // 根据页面类型提取特定的认证令牌
        match page_type {
            PageType::Grafana => {
                // 从 localStorage 中提取 Grafana 相关令牌
                if let Some(ref local_storage) = self.data.local_storage {
                    for (key, value) in local_storage {
                        if key.contains("grafana") || key.contains("auth") || key.contains("token") {
                            tokens.insert(key.clone(), value.clone());
                        }
                    }
                }
                
                // 从 cookies 中提取认证相关的
                for cookie in &cookies {
                    if cookie.name.contains("grafana") || 
                       cookie.name.contains("auth") ||
                       cookie.name.contains("session") ||
                       cookie.name.contains("token") {
                        tokens.insert(cookie.name.clone(), cookie.value.clone());
                    }
                }
            },
            PageType::Clinic => {
                // 从 localStorage 中提取 Clinic 相关令牌
                if let Some(ref local_storage) = self.data.local_storage {
                    for (key, value) in local_storage {
                        if key.contains("clinic") || 
                           key.contains("tidb") ||
                           key.contains("pingcap") ||
                           key.contains("auth") || 
                           key.contains("token") ||
                           key.contains("session") ||
                           key.contains("apikey") ||
                           key.contains("csrf") {
                            tokens.insert(key.clone(), value.clone());
                        }
                    }
                }
                
                // 从 cookies 中提取认证相关的
                for cookie in &cookies {
                    if cookie.name.contains("clinic") || 
                       cookie.name.contains("tidb") ||
                       cookie.name.contains("pingcap") ||
                       cookie.name.contains("auth") ||
                       cookie.name.contains("session") ||
                       cookie.name.contains("token") ||
                       cookie.name.contains("csrf") {
                        tokens.insert(cookie.name.clone(), cookie.value.clone());
                    }
                }
            },
            PageType::Unknown => {
                // 对于未知页面类型，提取通用的认证信息和常见服务的认证信息
                if let Some(ref local_storage) = self.data.local_storage {
                    for (key, value) in local_storage {
                        if key.contains("auth") || 
                           key.contains("token") ||
                           key.contains("session") ||
                           key.contains("clinic") || 
                           key.contains("tidb") ||
                           key.contains("pingcap") ||
                           key.contains("grafana") ||
                           key.contains("csrf") ||
                           key.contains("apikey") ||
                           key.contains("jwt") {
                            tokens.insert(key.clone(), value.clone());
                        }
                    }
                }
                
                for cookie in &cookies {
                    if cookie.name.contains("auth") ||
                       cookie.name.contains("session") ||
                       cookie.name.contains("token") ||
                       cookie.name.contains("clinic") || 
                       cookie.name.contains("tidb") ||
                       cookie.name.contains("pingcap") ||
                       cookie.name.contains("grafana") ||
                       cookie.name.contains("csrf") ||
                       cookie.name.contains("jwt") {
                        tokens.insert(cookie.name.clone(), cookie.value.clone());
                    }
                }
            }
        }
        
        AuthInfo {
            domain: self.domain.clone(),
            page_type,
            tokens,
            cookies,
            timestamp: self.timestamp,
        }
    }
}

impl CollectionResponse {
    /// 创建成功响应
    pub fn success(message: String, task_id: Option<String>) -> Self {
        Self {
            success: true,
            message,
            task_id,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    /// 创建错误响应
    pub fn error(message: String, task_id: Option<String>) -> Self {
        Self {
            success: false,
            message,
            task_id,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collection_request_validation() {
        let valid_request = CollectionRequest {
            url: "https://example.com".to_string(),
            domain: "example.com".to_string(),
            timestamp: 1234567890,
            task_id: None,
            data: CollectedData {
                cookies: "session=abc123".to_string(),
                page_type: "grafana".to_string(),
                user_agent: "Mozilla/5.0".to_string(),
                local_storage: None,
                session_storage: None,
            },
        };
        
        assert!(valid_request.validate().is_ok());
        
        let invalid_request = CollectionRequest {
            url: "".to_string(),
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
        
        assert!(invalid_request.validate().is_err());
    }
    
    #[test]
    fn test_extract_auth_info_grafana() {
        let mut local_storage = HashMap::new();
        local_storage.insert("grafana_auth_token".to_string(), "token123".to_string());
        
        let request = CollectionRequest {
            url: "https://grafana.example.com".to_string(),
            domain: "grafana.example.com".to_string(),
            timestamp: 1234567890,
            task_id: None,
            data: CollectedData {
                cookies: "grafana_session=session123".to_string(),
                page_type: "grafana".to_string(),
                user_agent: "Mozilla/5.0".to_string(),
                local_storage: Some(local_storage),
                session_storage: None,
            },
        };
        
        let auth_info = request.extract_auth_info();
        assert_eq!(auth_info.page_type, PageType::Grafana);
        assert!(auth_info.tokens.contains_key("grafana_auth_token"));
        assert!(auth_info.tokens.contains_key("grafana_session"));
    }
}