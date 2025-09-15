use rmcp::schemars;
use serde_aux::prelude::deserialize_bool_from_anything;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct LossyDdlRequest {
    pub sql: String,
    #[serde(default, deserialize_with = "deserialize_bool_from_anything")]
    pub collation: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct LossyDdlReport {
    pub sql: String,
    pub is_lossy: bool,
    pub reason: Option<String>,
}

// bool 的 Default 就是 false，但你可以自定义 struct 的 Default 实现
impl Default for LossyDdlRequest {
    fn default() -> Self {
        Self {
            sql: String::new(),
            collation: true,
        }
    }
}