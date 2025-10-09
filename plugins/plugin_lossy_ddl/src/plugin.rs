use crate::precheck_sql_with_collation;
use anyhow::Result;
use microkernel::platform::plugin_manager::Plugin;
use microkernel::{
    platform::message_bus::{BusMessage, HandlerMode, GLOBAL_MESSAGE_BUS},
    topic,
};
use serde_json::to_value;
use serde_json::Value;

fn fn_handler<F>(f: F) -> std::sync::Arc<dyn microkernel::platform::message_bus::MessageHandler>
where
    F: Fn(BusMessage) -> anyhow::Result<BusMessage> + Send + Sync + 'static,
{
    struct Handler<F: Fn(BusMessage) -> anyhow::Result<BusMessage> + Send + Sync + 'static>(F);
    #[async_trait::async_trait]
    impl<F> microkernel::platform::message_bus::MessageHandler for Handler<F>
    where
        F: Fn(BusMessage) -> anyhow::Result<BusMessage> + Send + Sync + 'static,
    {
        async fn handle(&self, msg: BusMessage) -> anyhow::Result<BusMessage> {
            (self.0)(msg)
        }
    }
    std::sync::Arc::new(Handler(f))
}

#[ctor::ctor]
fn init_lossy_ddl_plugin_bus() {
    use microkernel::platform::message_bus::BusClient;
    let topic = topic!("ddl_precheck");
    let handler_topic = topic.clone();
    let handler = move |msg: BusMessage| {
        let req = msg.data;
        let sql = req.get("sql").and_then(|v| v.as_str()).unwrap_or("");
        let collation_enabled = req
            .get("collation")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let result = precheck_sql_with_collation(sql, collation_enabled);
        let value = to_value(result)?;
        Ok(BusMessage::ok(handler_topic.clone(), value))
    };
    // 使用 BusClient 的同步注册，避免 tokio runtime 问题
    BusClient::new().register_request(topic, handler);
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
        vec!["ddl_precheck".to_string()]
    }
}
