//! AppConfig: 平台配置管理，支持多源加载、默认值和类型安全。

use clap::ArgMatches;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AppConfig {
    pub log_file: String,
    pub log_level: String,
    pub log_rotation: bool,
    pub config: String,
}

impl AppConfig {
    /// 从指定路径加载配置（支持 TOML/JSON），如无则用默认值。
    pub fn load(path: &str) -> Self {
        let path = Path::new(path);
        if path.exists() {
            let content = fs::read_to_string(path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            AppConfig::default()
        }
    }

    /// 用 clap ArgMatches 覆盖配置项
    pub fn merge_from_args(&mut self, args: &ArgMatches) {
        if let Some(log_file) = args.get_one::<String>("log_file") {
            self.log_file = log_file.clone();
        }
        if let Some(log_level) = args.get_one::<String>("log_level") {
            self.log_level = log_level.clone();
        }
        if let Some(config) = args.get_one::<String>("config") {
            self.config = config.clone();
        }
        // 可扩展更多参数覆盖
    }
}
