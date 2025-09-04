//! Microkernel platform layer for TiDB Intelligent Health Check (tihc).
//!
//! This module orchestrates plugin lifecycle, service discovery, and event-driven communication.
//! It provides the microkernel, plugin manager, service registry, event bus, and core services.
//! Platform layer: 微内核平台核心，负责 orchestrate 插件、服务、命令、消息总线。
//!
//! - plugin_manager: 插件生命周期管理。
//! - message_bus: 插件间异步消息/事件分发。
//! - core_services: 配置、日志、数据库等平台基础服务。

pub mod core_services;
pub mod message_bus;
pub mod plugin_manager;
pub use core_services::CoreServices;
pub use plugin_manager::PluginManager;
