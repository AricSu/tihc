use core::platform::command_registry::CommandRegistry;
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use core::infrastructure::{config, logging};
use crate::commands::slowlog::SlowlogOptions;
use crate::commands::web::WebOptions;
mod commands;

#[derive(Parser, Debug)]
#[command(
    name = "tihc",
    version = "1.0.0",
    author = "Aric <askaric@gmail.com>",
    about = "TiDB Intelligent Health Check (tihc) CLI Tool",
    long_about = "A CLI for TiDB Intelligent Health Check (tihc)\nDoc: https://www.askaric.com/en/tihc",
    after_help = "USAGE:\n    tihc [OPTIONS] <SUBCOMMAND>\n\nFor more information, visit: https://www.askaric.com/en/tihc"
)]
struct Cli {
    /// Log file path.
    #[arg(
        short = 'l',
        long = "log-file",
        global = true,
        required = false,
        help = "Log file path"
    )]
    log_file: Option<String>,
    /// Log level.
    #[arg(
        short = 'L',
        long = "log-level",
        global = true,
        required = false,
        default_value = "info"
    )]
    log_level: String,
    /// 是否启用日志切割
    #[arg(
        short = 'r',
        long = "enable-log-rotation",
        global = true,
        required = false,
        default_value_t = false
    )]
    enable_log_rotation: bool,
    /// Config file path.
    #[arg(
        short = 'c',
        long = "config",
        global = true,
        required = false,
        default_value = "config.toml",
        help = "Config file path"
    )]
    config_file: String,
    /// CLI subcommand.
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(subcommand, about = "Collect nessary info from TiDB components")]
    Tools(ToolsCommands),
    // #[clap(subcommand, about = "There are some commands for tuning and investigation")]
    // Collect(CollectCommands),
    #[clap(about = "Start the HTTP server for dashboard integration")]
    Server(WebOptions),
}

#[derive(Subcommand, Debug)]
pub enum ToolsCommands {
    #[clap(about = "Parse TiDB slow log file and import to database")]
    Slowlog(SlowlogOptions),
    // #[clap(about = "Get issue info from GitHub")]
    // BugInfo(BugInfoOptions),
}

fn register_all_plugins(kernel: &mut core::platform::Microkernel, command_registry: &mut CommandRegistry) {
    use plugin_slowlog::SlowLogPlugin;
    use plugin_sql_editor::SqlEditorPlugin;
    let mut ctx = core::plugin_api::traits::PluginContext {
        service_registry: kernel.service_registry.clone(),
        command_registry: Some(unsafe { std::mem::transmute::<&mut CommandRegistry, &'static mut CommandRegistry>(command_registry) }),
    };
    kernel.plugin_manager.register_plugin(Box::new(SlowLogPlugin), &mut ctx);
    kernel.plugin_manager.register_plugin(Box::new(SqlEditorPlugin), &mut ctx);
}


/// Main entry point for TiDB Intelligent Health Check (tihc) CLI/Web.
///
/// - CLI: tihc [OPTIONS] <SUBCOMMAND>
/// - Web: tihc server --host 127.0.0.1 --port 5000
// ...existing code...
fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = &cli.config_file;
    // === 配置加载与日志初始化（合并 CLI/config，CLI 优先） ===
    let app_config = match config::load_config(config_path) {
        Ok(cfg) => {
            tracing::info!("Loaded config: {:?}", cfg);
            cfg
        }
        Err(e) => {
            tracing::warn!("Config not loaded: {} (path={})", e, config_path);
            config::AppConfig {
                some_option: None,
                enable_log_rotation: None,
                log_file: None,
                log_level: None,
            }
        }
    };
    let merged = config::MergedConfig::from(
        &cli.log_level,
        cli.log_file.as_deref(),
        cli.enable_log_rotation,
        &app_config,
    );

    logging::init_logging(
        merged.log_file.as_ref(),
        merged.log_level.as_ref(),
        merged.enable_log_rotation,
    )?;

    let mut kernel = core::platform::Microkernel::new(app_config.clone());
    let mut command_registry = CommandRegistry::new();
    register_all_plugins(&mut kernel, &mut command_registry);
    // === 演示 handler 层访问全局配置（只打印一次即可） ===
    {
        use tracing::info;
        let app_config = kernel.core_services.config_service.get();
        info!(target: "tihc", "[demo] config.some_option={:?}", app_config.some_option);
    }
    match &cli.command {
        Some(Commands::Tools(tools_cmd)) => {
            let (cmd, args) = match tools_cmd {
                ToolsCommands::Slowlog(opts) => {
                    let mut args = Vec::new();
                    args.push(opts.log_dir.clone());
                    args.push(opts.pattern.clone());
                    ("slowlog-scan", args)
                }
            };
            command_registry.execute(cmd, &args)?;
        }
        Some(Commands::Server(web_opts)) => {
            // Start the web server for dashboard integration
            let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
            // 注入 command_registry 到 web 服务
            rt.block_on(commands::web::start_web_service(web_opts, command_registry))?;
        }
        None => {
            Cli::command().print_help()?;
            println!();
        }
    }
    Ok(())
}
