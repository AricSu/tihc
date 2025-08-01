use serde::Deserialize;
use std::{borrow::Cow, fs};

/// 全局配置结构体（可扩展）
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub some_option: Option<String>,
    /// 日志级别（如 "info"、"debug"），可被 CLI 覆盖
    pub log_level: Option<String>,
    /// 日志文件路径，可被 CLI 覆盖
    pub log_file: Option<String>,
    /// 是否启用日志切割，可被 CLI 覆盖
    pub enable_log_rotation: Option<bool>,
    // 其他配置项...
}

/// 合并 CLI 参数和 config，CLI 优先，config 兜底
pub struct MergedConfig<'a> {
    pub log_level: Cow<'a, str>,
    pub log_file: Cow<'a, str>,
    pub enable_log_rotation: bool,
}

impl<'a> MergedConfig<'a> {
    /// 构造合并后的配置
    pub fn from(
        cli_log_level: &'a str,
        cli_log_file: Option<&'a str>,
        cli_enable_log_rotation: bool,
        config: &'a AppConfig,
    ) -> Self {
        let log_level = if !cli_log_level.is_empty() {
            Cow::Borrowed(cli_log_level)
        } else if let Some(l) = config.log_level.as_ref() {
            Cow::Borrowed(l.as_str())
        } else {
            Cow::Borrowed("info")
        };

        let log_file = if let Some(cli_file) = cli_log_file {
            Cow::Borrowed(cli_file)
        } else if let Some(cfg_file) = config.log_file.as_ref() {
            Cow::Borrowed(cfg_file.as_str())
        } else {
            Cow::Borrowed("tihc.log")
        };

        let enable_log_rotation = if cli_enable_log_rotation {
            true
        } else {
            config.enable_log_rotation.unwrap_or(false)
        };

        Self {
            log_level,
            log_file,
            enable_log_rotation,
        }
    }
}

/// 从 TOML 文件加载配置
pub fn load_config(path: &str) -> anyhow::Result<AppConfig> {
    let content = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&content)?;
    Ok(config)
}
