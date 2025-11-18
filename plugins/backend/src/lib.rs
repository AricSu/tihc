use microkernel::plugin::{KernelPlugin, PluginEvent, PluginRegistry};
use std::sync::Arc;

pub struct BackendPlugin;

impl KernelPlugin for BackendPlugin {
    fn register(&self, bus: Arc<microkernel::EventBus<PluginEvent>>, registry: Arc<PluginRegistry>) {
        tracing::info!(target: "backend", "BackendPlugin::register called");
        crate::startup::register_static_embed_via_bus(bus, registry);
    }
}

fn backend_plugin_factory() -> Box<dyn KernelPlugin> {
    Box::new(BackendPlugin)
}

inventory::submit! {
    microkernel::plugin::PluginFactory(backend_plugin_factory)
}
mod static_embed;

pub use static_embed::static_dist_router;
pub mod startup;
pub mod interface;
pub mod infrastructure;
pub mod application;
pub mod domain;

pub use startup::*;