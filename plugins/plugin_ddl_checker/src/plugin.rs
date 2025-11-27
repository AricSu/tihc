
use inventory;
use microkernel::plugin::{KernelPlugin, PluginFactory, PluginEvent, PluginHandler, PluginRegistry, RegisterHttpRoute};
use crate::precheck_sql_with_collation;
use microkernel::event_bus::{EventBus, EventEnvelope};
use std::sync::Arc;
use crate::types::AnalysisResult;



pub fn ddl_plugin_factory() -> Box<dyn KernelPlugin> {
    Box::new(LossyDDLPlugin::default())
}

inventory::submit! {
    PluginFactory(ddl_plugin_factory)
}



#[derive(Debug, Default)]
pub struct LossyDDLPlugin {
    pub name: String,
    pub description: String,
}






impl KernelPlugin for LossyDDLPlugin {
    fn register(&self, bus: Arc<EventBus<PluginEvent>>, registry: Arc<PluginRegistry>) {
        tracing::info!(target: "plugin_ddl_checker::plugin", "[DDL_PLUGIN] Registering /ddl/precheck route...");
        let handler: PluginHandler = Arc::new(move |_req| {
            Box::pin(async move {
                axum::response::Response::new(axum::body::Body::from("[ddl_checker] response"))
            })
        });
        let path = "/ddl/precheck".to_string();
        registry.routes.insert(path.clone(), handler.clone());
        let event = EventEnvelope::new(
            "plugin_register_http_route",
            PluginEvent::RegisterHttpRoute(RegisterHttpRoute { path }),
            None,
        );
        let _ = bus.broadcast(event);
        tracing::info!(target: "plugin_ddl_checker::plugin", "[DDL_PLUGIN] /ddl/precheck route registered!");

        // 订阅 ddl_precheck 请求事件，实现 DDL 检查分发
        let bus_stream = bus.subscribe();
        let bus_clone = bus.clone();
        tokio::spawn(async move {
            let mut receiver = bus_stream;
            tracing::info!(target: "plugin_ddl_checker::plugin", "[DDL_PLUGIN] EventBus subscription started");
            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        tracing::info!(target: "plugin_ddl_checker::plugin", "[DDL_PLUGIN] Received event: {}", event.event_type);
                        if event.event_type == "ddl_precheck" {
                            let req_val = match &event.payload {
                                PluginEvent::Custom(_, v) => v.clone(),
                                _ => serde_json::to_value(&event.payload).unwrap_or_default(),
                            };
                            let sql = req_val.get("sql").and_then(|v| v.as_str()).unwrap_or("");
                            let collation_enabled = req_val.get("collation").and_then(|v| v.as_bool()).unwrap_or(false);
                            let result: AnalysisResult = precheck_sql_with_collation(sql, collation_enabled);
                            let result_value = serde_json::to_value(result).unwrap_or_default();
                            let event_payload = PluginEvent::Custom("ddl_precheck_result".to_string(), result_value);
                            let result_event = EventEnvelope::new(
                                "ddl_precheck_result",
                                event_payload,
                                None,
                            );
                            tracing::info!(target: "plugin_ddl_checker::plugin", "[DDL_PLUGIN] Broadcasting result event");
                            let _ = bus_clone.broadcast(result_event);
                        }
                    }
                    Err(e) => {
                        tracing::error!(target: "plugin_ddl_checker::plugin", "[DDL_PLUGIN] EventBus recv error: {}", e);
                    }
                }
            }
        });
    }
}
