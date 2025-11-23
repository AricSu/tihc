mod startup;
mod interface;
mod infrastructure;
mod domain;
mod application;

use microkernel::plugin::{KernelPlugin, PluginEvent, PluginRegistry};
use std::sync::Arc;

struct BackendPlugin;

impl KernelPlugin for BackendPlugin {
    fn register(&self, bus: Arc<microkernel::EventBus<PluginEvent>>, registry: Arc<PluginRegistry>) {
        // 注册静态资源路由
        crate::startup::register_static_embed_via_bus(bus.clone(), registry.clone());
        // 可在此扩展更多插件注册逻辑
        // 例如：
        // crate::startup::register_other_plugin_via_bus(bus, registry);
    }
}

fn backend_plugin_factory() -> Box<dyn KernelPlugin> {
    Box::new(BackendPlugin)
}

inventory::submit! {
    microkernel::plugin::PluginFactory(backend_plugin_factory)
}