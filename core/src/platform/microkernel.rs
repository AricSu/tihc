//! Microkernel orchestrates the platform, managing plugin lifecycle, service registry, and event bus.
use super::core_services::CoreServices;
use super::event_bus::EventBus;
use super::plugin_manager::PluginManager;
use super::service_registry::ServiceRegistry;
use crate::infrastructure::config::AppConfig;
use crate::platform::CommandRegistry;

/// The Microkernel is the central orchestrator of the system.
///
/// Manages plugin lifecycle, service registration, and event dispatching.
/// Provides the runtime environment for plugins and core services.
pub struct Microkernel {
    pub service_registry: std::sync::Arc<std::sync::Mutex<ServiceRegistry>>,
    pub command_registry: CommandRegistry,
    pub event_bus: EventBus,
    pub plugin_manager: PluginManager,
    pub core_services: CoreServices,
}

impl Microkernel {
    /// Creates a new Microkernel instance with all core services initialized and config injected.
    pub fn new(config: AppConfig) -> Self {
        Microkernel {
            service_registry: std::sync::Arc::new(std::sync::Mutex::new(ServiceRegistry::new())),
            command_registry: CommandRegistry::new(),
            event_bus: EventBus::new(),
            plugin_manager: PluginManager::new(),
            core_services: CoreServices::new(config),
        }
    }
}
