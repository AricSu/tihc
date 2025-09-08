use microkernel::platform::message_bus::{BusMessage, GLOBAL_MESSAGE_BUS, HandlerMode};
use serde_json::Value;
use anyhow::Result;
use crate::precheck_sql_with_collation;
use microkernel::platform::plugin_manager::Plugin;


#[ctor::ctor]
fn init_lossy_ddl_plugin_bus() {
    GLOBAL_MESSAGE_BUS.clone().register_fn(
        "ddl_precheck",
        |msg| {
            let req = msg.unwrap_data::<Value>()?;
            let sql = req.get("sql").and_then(|v| v.as_str()).unwrap_or("");
            let collation_enabled = req.get("collation_enabled").and_then(|v| v.as_bool()).unwrap_or(false);
            let result = precheck_sql_with_collation(sql, collation_enabled);
            Ok(BusMessage::ok("ddl_precheck", result))
        },
        Some(HandlerMode::Request)
    );
}

#[derive(Debug, Default)]
pub struct LossyDDLPlugin {
    pub name: String,
    pub description: String,
}

impl LossyDDLPlugin {
    pub fn new() -> Self {
        Self {
            name: "lossy_ddl".to_string(),
            description: "Lossy DDL detection plugin".to_string(),
        }
    }
}

impl Plugin for LossyDDLPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }

    fn on_shutdown(&self, _msg: &BusMessage) -> Result<()> {
        tracing::info!(target: "lossy_ddl", "LossyDDLPlugin received shutdown signal, cleaning up...");
        Ok(())
    }

    fn topics(&self) -> Vec<String> {
        vec!["ddl-precheck".to_string()]
    }
}
