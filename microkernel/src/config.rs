//! microkernel 配置模块，支持从文件/env 加载配置。
use serde::Deserialize;
use std::collections::HashMap;
use toml::Value;
use std::{fs, path::Path};

/// Kernel configuration for microkernel core.


#[derive(Debug, Clone, Deserialize)]
pub struct KernelConfig {
    /// Server configuration ([server] table)
    #[serde(default)]
    pub server: ServerConfig,

    /// Logging configuration ([log] table)
    #[serde(default)]
    pub log: LogConfig,

    /// Kernel section ([kernel] table)
    #[serde(default)]
    pub kernel: KernelSection,

    /// Plugins configuration ([plugins] table, dynamic)
    #[serde(default)]
    pub plugins: HashMap<String, Value>,

    /// Config file path (for reload/self-inspect)
    #[serde(default = "KernelConfig::default_config_file")]
    pub config_file: String,
}

// PluginsConfig is now a dynamic HashMap<String, toml::Value>.
// Each plugin can access its config via: config.plugins.get("autoflow")
// and use serde or toml::Value API to parse its own config.

#[derive(Debug, Clone, Deserialize)]
pub struct KernelSection {
    /// EventBus broadcast channel capacity
    #[serde(default = "KernelSection::default_broadcast_capacity")]
    pub eventbus_broadcast_capacity: usize,
    /// EventBus RPC channel capacity
    #[serde(default = "KernelSection::default_rpc_capacity")]
    pub eventbus_rpc_capacity: usize,
    // Extend with more kernel/core settings as needed
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// Enable the server (default: true)
    #[serde(default = "ServerConfig::default_enable")]
    pub enable: bool,
    /// Listen address, e.g. "0.0.0.0"
    #[serde(default = "ServerConfig::default_address")]
    pub address: String,
    /// Listen port, e.g. 8080
    #[serde(default = "ServerConfig::default_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    /// Log level (e.g. "info", "debug", "warn", "error")
    #[serde(default = "LogConfig::default_level")]
    pub level: String,
    /// Log file path (if None, log to stdout)
    #[serde(default)]
    pub file: Option<String>,
    /// Enable log rotation (default: false)
    #[serde(default = "LogConfig::default_enable_rotation")]
    pub enable_rotation: bool,
    // Extend with more log options as needed (format, rotation, etc)
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            log: LogConfig::default(),
            kernel: KernelSection::default(),
            plugins: HashMap::new(),
            config_file: Self::default_config_file(),
        }
    }
}

impl Default for KernelSection {
    fn default() -> Self {
        Self {
            eventbus_broadcast_capacity: Self::default_broadcast_capacity(),
            eventbus_rpc_capacity: Self::default_rpc_capacity(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            enable: Self::default_enable(),
            address: Self::default_address(),
            port: Self::default_port(),
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Self::default_level(),
            file: Some(Self::default_file()),
            enable_rotation: Self::default_enable_rotation(),
        }
    }
}

impl KernelConfig {
    fn default_config_file() -> String {
        "config.toml".to_string()
    }

    /// Load config from TOML file, fallback to default if file missing or parse error.
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).or_else(|_| Ok(Self::default())),
            Err(_) => Ok(Self::default()),
        }
    }
}

impl KernelSection {
    fn default_broadcast_capacity() -> usize {
        1024
    }
    fn default_rpc_capacity() -> usize {
        256
    }
}

impl ServerConfig {
    fn default_enable() -> bool {
        true
    }
    fn default_address() -> String {
        "0.0.0.0".to_string()
    }
    fn default_port() -> u16 {
        8080
    }
}

impl LogConfig {
    fn default_level() -> String {
        "info".to_string()
    }
    fn default_file() -> String {
        "tihc.log".to_string()
    }
    fn default_enable_rotation() -> bool {
        false
    }
}