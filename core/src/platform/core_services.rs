//! Core services provided by the system, such as logging, database connection, configuration management, and graceful shutdown.
use crate::infrastructure::config::AppConfig;

#[derive(Clone)]
pub struct CoreServices {
    pub config_service: ConfigService,
    pub logging_service: LoggingService,
    pub database_service: DatabaseService,
    pub shutdown_service: tokio::sync::broadcast::Sender<()>,
}

impl CoreServices {
    /// Creates a new CoreServices instance with config injected.
    pub fn new(config: AppConfig) -> Self {
        let (shutdown_service, _) = tokio::sync::broadcast::channel(8);
        CoreServices {
            config_service: ConfigService::new(config),
            logging_service: LoggingService::new(),
            database_service: DatabaseService::new(),
            shutdown_service,
        }
    }
    /// Broadcasts shutdown signal.
    pub fn broadcast_shutdown(&self) {
        let _ = self.shutdown_service.send(());
    }
    /// 获取 shutdown 信号订阅者（供异步任务使用）
    pub fn subscribe_shutdown(&self) -> tokio::sync::broadcast::Receiver<()> {
        self.shutdown_service.subscribe()
    }
}

/// Manages the configuration settings of the system.
/// Now holds the full AppConfig struct.
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct DatabaseService;

impl DatabaseService {
    /// Creates a new DatabaseService.
    pub fn new() -> Self {
        DatabaseService
    }
    // TODO: Implement database connection logic.
}
