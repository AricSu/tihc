//! CoreServices: 平台基础服务集合，负责配置、日志、数据库等核心能力的统一管理与注入。
//!
//! 设计目标：
//! - 为微内核和插件提供统一的基础服务访问入口。
//! - 支持配置管理、日志、数据库连接、监控等扩展。
//!
//! 用法示例：
//! let core = CoreServices::new(config);
//! let db = core.db();
//! let logger = core.logger();

use super::plugin_manager::PluginManager;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::logging::Logger;

/// CoreServices: 平台基础服务集合。
pub struct CoreServices {
    config: AppConfig,
    logger: Logger,
    plugin_manager: PluginManager,
    // 可扩展更多服务，如 metrics、cache 等
}

impl CoreServices {
    /// 获取插件管理器的可变引用（支持动态注册/卸载）。
    pub fn plugin_manager_mut(&mut self) -> &mut PluginManager {
        &mut self.plugin_manager
    }
    /// 创建新的 CoreServices 实例。
    pub fn new(config: AppConfig) -> Self {
        let logger = Logger::new(&config);
        let plugin_manager = PluginManager::new();
        CoreServices {
            config,
            logger,
            plugin_manager,
        }
    }

    /// 获取配置服务。
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// 获取日志服务。
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    /// 获取插件管理器。
    pub fn plugin_manager(&self) -> &PluginManager {
        &self.plugin_manager
    }
}
