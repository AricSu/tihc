use crate::domain::counter::Counter;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StructRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExamplePromptArgs {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CounterAnalysisArgs {
    pub goal: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Clone)]
pub struct CounterService {
    pub counter: Counter,
}

impl CounterService {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }
}
