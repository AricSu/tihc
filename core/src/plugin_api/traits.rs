use std::sync::Arc;

// Ensure CommandHandler trait is available
use crate::platform::command_registry::CommandHandler;
/// Defines the plugin interface for the microkernel platform.
///
/// Plugins must implement this trait to be discovered, registered,
/// and managed by the core platform. Registration should include
/// service trait implementations and command/event handlers.
///
/// # Examples
/// ```
/// pub struct LossyDdlPlugin;
///
/// impl Plugin for LossyDdlPlugin {
///     fn name(&self) -> &str { "lossy_ddl" }
///     fn register(&mut self, ctx: &mut PluginContext) {
///         // Register commands and services here.
///     }
/// }
/// ```
pub trait Plugin {
    /// Returns the unique name of the plugin.
    ///
    /// This name is used for plugin discovery and management.
    fn name(&self) -> &str;

    /// Registers the plugin with the runtime context.
    /// Implementors should register service trait implementations, command handlers, and event subscriptions as needed.
    fn register(&mut self, ctx: &mut PluginContext);
}

/// Provides runtime context for plugin registration and command/service/event handling.
pub struct PluginContext {
    pub service_registry: Arc<std::sync::Mutex<crate::platform::service_registry::ServiceRegistry>>,
    pub command_registry: Option<&'static mut crate::platform::command_registry::CommandRegistry>,
}

impl PluginContext {
    /// Registers a command handler with the platform's command registry.
    pub fn register_command(&mut self, name: &str, handler: Box<dyn CommandHandler>) {
        if let Some(registry) = &mut self.command_registry {
            registry.register(name, handler);
        }
    }
}
