//! Interface layer for TiDB Intelligent Health Check (tihc).
//!
//! This module adapts core services for external interaction and integration.
//! It provides CLI and Web API adapters for user and system access.
pub mod cli;

pub fn start_interface() {
    println!("Starting CLI or Web interface");
}
