//! Application layer for TiDB Intelligent Health Check (tihc).
//!
//! This module coordinates use cases, domain services, and business logic.
//! It orchestrates interactions between domain models and external systems.
pub mod entity;

pub struct ExampleDomainModel {
    pub id: u64,
    pub name: String,
}
