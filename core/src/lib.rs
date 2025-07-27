//! TiDB Intelligent Health Check (tihc) — Core crate entry.
//!
//! This crate implements the microkernel platform and DDD layers for plugin-based TiDB diagnostics.
//!
//! # Architecture
//! - Microkernel platform: Plugin lifecycle, service registry, event bus, core services.
//! - DDD layers:
//!   - domain: Domain models, entities, events, business rules.
//!   - application: UseCase services, business logic coordination.
//!   - infrastructure: External systems, database, adapters.
//!   - interface: CLI/Web API adapters.
//! - plugin_api: Plugin trait definitions and registration interfaces.
//!
//! # Design Principles
//! - Plugins are DDD bounded contexts, high cohesion, low coupling.
//! - Microkernel only handles orchestration, registration, logging, configuration.
//! - Plugin communication via service registry and trait interfaces.
//! - All modules are independently testable and support self-contained builds.

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod platform;
pub mod plugin_api;
