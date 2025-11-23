inventory::submit! {
    microkernel::plugin::PluginFactory(backend_plugin_factory)
}
mod startup;
mod interface;
mod infrastructure;
mod domain;
mod application;

use microkernel::plugin::{KernelPlugin, PluginEvent, PluginRegistry};
use std::sync::Arc;
use tokio::task;

struct BackendPlugin;

impl KernelPlugin for BackendPlugin {
    fn register(&self, bus: Arc<microkernel::EventBus<PluginEvent>>, registry: Arc<PluginRegistry>) {
        use microkernel::context::get_global_config;
        let config = get_global_config();
        let backend_cfg = config.plugins.get("backend").expect("Missing [plugins.backend] config");
        let backend_cfg = backend_cfg.clone();
        task::spawn(async move {
            startup::register_static_embed_via_bus(bus.clone(), registry.clone());
            startup::register_api_routes_via_bus(bus.clone(), registry.clone(), &backend_cfg).await;
        });
    }
}

pub fn backend_plugin_factory() -> Box<dyn KernelPlugin> {
    Box::new(BackendPlugin)
}
