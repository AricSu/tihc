//! Domain layer for TiDB Intelligent Health Check (tihc).
//!
//! This module contains domain models, entities, events, and business rules.
//! It defines the core business logic and invariants for the system.
pub mod service;

pub struct ExampleService;

impl ExampleService {
    pub fn do_something(&self) {
        println!("Application service doing something");
    }
}
