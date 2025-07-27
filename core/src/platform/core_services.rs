//! Core services provided by the system, such as logging, database connection, and configuration management.
use crate::infrastructure::config::AppConfig;
use std::collections::HashMap;

pub struct CoreServices {
    pub config_service: ConfigService,
    pub logging_service: LoggingService,
    pub database_service: DatabaseService,
}

impl CoreServices {
    /// Creates a new CoreServices instance with config injected.
    pub fn new(config: AppConfig) -> Self {
        CoreServices {
            config_service: ConfigService::new(config),
            logging_service: LoggingService::new(),
            database_service: DatabaseService::new(),
        }
    }
}

/// Manages the configuration settings of the system.
/// Now holds the full AppConfig struct.
pub struct ConfigService {
    pub app_config: AppConfig,
}

impl ConfigService {
    /// Creates a new ConfigService with config injected.
    pub fn new(app_config: AppConfig) -> Self {
        ConfigService { app_config }
    }

    /// Gets a reference to the full AppConfig.
    pub fn get(&self) -> &AppConfig {
        &self.app_config
    }
}

/// Provides logging functionality for the system, supporting various log levels.
pub struct LoggingService;

impl LoggingService {
    /// Creates a new LoggingService.
    pub fn new() -> Self {
        LoggingService
    }

    /// Logs an info message.
    pub fn log_info(&self, message: &str) {
        println!("[INFO] {}", message);
    }

    /// Logs an error message.
    pub fn log_error(&self, message: &str) {
        println!("[ERROR] {}", message);
    }
}

/// Provides database connection management for plugins and core services.
pub struct DatabaseService;

impl DatabaseService {
    /// Creates a new DatabaseService.
    pub fn new() -> Self {
        DatabaseService
    }
    // TODO: Implement database connection logic.
}
