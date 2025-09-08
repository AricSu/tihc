use anyhow::Result;
use infrastructure::config::AppConfig;
use std::path::Path;
use tracing_subscriber::fmt;

use crate::infrastructure;
pub struct Logger {
    log_file: String,
    log_level: String,
    enable_rotation: bool,
}

impl Logger {
    pub fn new(config: &AppConfig) -> Self {
        Logger {
            log_file: config.log_file.clone(),
            log_level: config.log_level.clone(),
            enable_rotation: config.log_rotation,
        }
    }

    /// 初始化全局日志配置。
    pub fn init(&self) -> Result<()> {
        let log_path = Path::new(&self.log_file);
        // 路径合法性检查
        if self.log_file.trim().is_empty() {
            return Err(anyhow::anyhow!("Log file path is empty"));
        }
        if let Some(parent) = log_path.parent() {
            if parent.as_os_str().is_empty() {
                return Err(anyhow::anyhow!(
                    "Log file parent directory is empty or invalid"
                ));
            }
            std::fs::create_dir_all(parent)?;
        } else {
            return Err(anyhow::anyhow!("Log file path has no parent directory"));
        }
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.log_file.clone())?;
        let level = match self.log_level.to_ascii_lowercase().as_str() {
            "error" => tracing::Level::ERROR,
            "warn" | "warning" => tracing::Level::WARN,
            "debug" => tracing::Level::DEBUG,
            "trace" => tracing::Level::TRACE,
            _ => tracing::Level::INFO,
        };
        fmt()
            .with_writer(file)
            .with_max_level(level)
            .with_ansi(false)
            .with_timer(tracing_subscriber::fmt::time())
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .init();
        Ok(())
    }

    /// 输出自定义欢迎信息
    pub fn welcome(messages: &[&str]) {
        for msg in messages {
            tracing::info!(target: "tihc", "{}", msg);
        }
    }
}
