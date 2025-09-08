// Settings Application Service

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use crate::domain::settings::{Language, AppSettings};
use crate::domain::shared::{DomainError, DomainResult};

/// 语言设置请求
#[derive(Deserialize)]
pub struct SetLanguageRequest {
    pub lang: String,
}

/// 语言设置响应
#[derive(Serialize)]
pub struct LanguageResponse {
    pub lang: String,
}

/// 设置应用服务接口
#[async_trait]
pub trait SettingsApplicationService: Send + Sync {
    /// 获取当前语言设置
    async fn get_language(&self) -> DomainResult<LanguageResponse>;
    
    /// 设置语言
    async fn set_language(&self, request: SetLanguageRequest) -> DomainResult<()>;
    
    /// 获取支持的语言列表
    async fn get_supported_languages(&self) -> DomainResult<Vec<String>>;
}

/// 设置应用服务实现
pub struct SettingsApplicationServiceImpl {
    // 在实际应用中，这里应该使用持久化存储
    // 现在为了兼容性，暂时使用内存存储
    settings: Arc<Mutex<AppSettings>>,
}

impl SettingsApplicationServiceImpl {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(Mutex::new(AppSettings::default())),
        }
    }
}

#[async_trait]
impl SettingsApplicationService for SettingsApplicationServiceImpl {
    async fn get_language(&self) -> DomainResult<LanguageResponse> {
        let settings = self.settings.lock().map_err(|e| DomainError::InternalError {
            message: format!("Failed to acquire settings lock: {}", e),
        })?;
        
        Ok(LanguageResponse {
            lang: settings.language.to_string(),
        })
    }
    
    async fn set_language(&self, request: SetLanguageRequest) -> DomainResult<()> {
        let language = Language::from_str(&request.lang)?;
        
        let mut settings = self.settings.lock().map_err(|e| DomainError::InternalError {
            message: format!("Failed to acquire settings lock: {}", e),
        })?;
        
        settings.update_language(language);
        Ok(())
    }
    
    async fn get_supported_languages(&self) -> DomainResult<Vec<String>> {
        let supported = Language::supported_languages()
            .into_iter()
            .map(|lang| lang.to_string())
            .collect();
            
        Ok(supported)
    }
}
