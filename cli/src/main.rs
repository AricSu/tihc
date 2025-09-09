use crate::commands::Cli;
use crate::commands::Commands;
mod check_gcc;
use anyhow::Result;
use clap::{CommandFactory, Parser};
use microkernel::infrastructure::config;
mod commands;
mod plugin;
use config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // 启动时检测 gcc（仅 Linux glibc 构建有效）
    check_gcc::check_gcc();
    let cli = Cli::parse();

    // 打印通用欢迎信息到日志
    microkernel::infrastructure::logging::Logger::welcome(&[
        &format!(
            "🎯 Welcome to use TiDB Health Check (tihc) v{} starting",
            env!("CARGO_PKG_VERSION")
        ),
        "📖 Documentation: https://www.askaric.com/en/tihc",
        "👨‍💻 Author: Aric <ask.aric.su@gmail.com>",
    ]);

    let mut app_config = AppConfig::load(&cli.config_file);
    // 优先用 CLI 参数覆盖配置文件
    if let Some(log_file) = &cli.log_file {
        tracing::debug!(target: "config", "Override log_file from CLI: {}", log_file);
        app_config.log_file = log_file.clone();
    }
    tracing::debug!(target: "config", "Override log_level from CLI: {}", cli.log_level);
    app_config.log_level = cli.log_level.clone();
    tracing::debug!(target: "config", "Override log_rotation from CLI: {}", cli.enable_log_rotation);
    app_config.log_rotation = cli.enable_log_rotation;
    tracing::debug!(target: "config", "Override config path from CLI: {}", cli.config_file);
    app_config.config = cli.config_file.clone();

    tracing::info!(target: "tihc", "[core_services] Loaded config: {:?}", app_config);

    // 保证 log_file 字段始终有默认值
    if app_config.log_file.trim().is_empty() {
        app_config.log_file = "/Users/aric/Database/tihc/tihc.log".to_string();
        tracing::warn!(target: "config", "log_file is empty, using default: tihc.log");
    }
    let mut core_services = microkernel::platform::CoreServices::new(app_config.clone());
    // 初始化日志系统
    core_services.logger().init()?;
    // 注册所有插件（支持动态注册）
    plugin::register_plugins(core_services.plugin_manager_mut()).await;

    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Tools(tools_cmd) => {
                commands::handle_tools_command(&tools_cmd).await?;
            }
            Commands::Server(web_opts) => {
                commands::handle_server_command(web_opts.clone()).await?;
            }
        }
    } else {
        Cli::command().print_help()?;
        println!();
    }
    Ok(())
}
