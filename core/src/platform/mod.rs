//! Microkernel platform layer for TiDB Intelligent Health Check (tihc).
//!
//! This module orchestrates plugin lifecycle, service discovery, and event-driven communication.
//! It provides the microkernel, plugin manager, service registry, event bus, and core services.
pub mod command_registry;
pub mod core_services;
pub mod event_bus;
pub mod microkernel;
pub mod plugin_manager;
pub mod service_registry;
pub use command_registry::CommandRegistry;
pub use event_bus::EventBus;
pub use microkernel::Microkernel;
pub use plugin_manager::PluginManager;
pub use service_registry::ServiceRegistry;

pub fn start_platform() {
    println!("Microkernel platform starting");
}
