use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoflowConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
    pub default_engine: String,
}

impl Default for AutoflowConfig {
    fn default() -> Self {
        Self {
            base_url: "https://tidb.ai/api/v1".to_string(),
            api_key: None,
            timeout_seconds: 30,
            default_engine: "tidbcloud-chatbot".to_string(),
        }
    }
}
