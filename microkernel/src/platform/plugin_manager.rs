//! PluginManager handles plugin loading, registration, and lifecycle management.

use crate::plugin_api::traits::Plugin;
use std::collections::HashMap;

/// The PluginManager handles the loading and lifecycle of plugins.
/// It ensures that plugins are registered correctly and can be invoked by other parts of the system.
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    /// Creates a new PluginManager.
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    /// Registers a plugin with the manager.
    /// Only responsible for exposing capabilities and lifecycle hooks.
    pub fn register_plugin(
        &mut self,
        plugin: Box<dyn Plugin>,
        ctx: &mut crate::plugin_api::traits::PluginContext,
    ) {
        let mut plugin = plugin;
        plugin.register(ctx);
        self.plugins.insert(plugin.name().to_string(), plugin);
    }

    /// Retrieves a plugin by name.
    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(name)
    }
}
