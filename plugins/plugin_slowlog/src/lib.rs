/// Domain layer: slowlog domain models, entities, rules
pub mod domain;
/// Infrastructure layer: file parsing, external dependencies
pub mod infrastructure;
/// Application layer: use case services, business logic
pub mod application;
/// Interface layer: CLI/Web adapters
pub mod interface;
/// Plugin registration and entry
pub mod plugin;

/// Plugin main entry, for microkernel platform discovery and registration
pub use plugin::SlowLogPlugin;
