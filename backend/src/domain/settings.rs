// Settings Domain - 语言和配置管理

use serde::{Deserialize, Serialize};
use crate::domain::shared::DomainError;

/// 支持的语言列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "en")]
    English,
}

impl Language {
    /// 获取所有支持的语言
    pub fn supported_languages() -> Vec<Language> {
        vec![Language::Chinese, Language::English]
    }
    
    /// 从字符串创建语言实例
    pub fn from_str(lang: &str) -> Result<Self, DomainError> {
        match lang {
            "zh" => Ok(Language::Chinese),
            "en" => Ok(Language::English),
            _ => Err(DomainError::ValidationError {
                message: format!("Unsupported language: {}. Supported languages: zh, en", lang)
            }),
        }
    }
    
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            Language::Chinese => "zh".to_string(),
            Language::English => "en".to_string(),
        }
    }
    
    /// 获取默认语言
    pub fn default() -> Self {
        Language::English
    }
}

/// 应用设置值对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: Language,
}

impl AppSettings {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
    
    /// 创建默认设置
    pub fn default() -> Self {
        Self {
            language: Language::default(),
        }
    }
    
    /// 更新语言设置
    pub fn update_language(&mut self, language: Language) {
        self.language = language;
    }
}
