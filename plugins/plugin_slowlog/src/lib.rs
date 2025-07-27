/// Application layer: use case services, business logic
pub mod application;
/// Domain layer: slowlog domain models, entities, rules
pub mod domain;
/// Infrastructure layer: file parsing, external dependencies
pub mod infrastructure;
/// Plugin registration and entry
pub mod plugin;
/// Interface layer: handler, trait re-export
pub mod interface;

/// Plugin main entry, for microkernel platform discovery and registration
pub use plugin::SlowLogPlugin;

