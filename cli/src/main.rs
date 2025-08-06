use crate::commands::slowlog::SlowlogOptions;
use crate::commands::web::WebOptions;
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use microkernel::infrastructure::{config, logging};
use microkernel::platform::command_registry::CommandRegistry;
mod commands;
use plugin_slowlog::SlowLogPlugin;
use plugin_sql_editor::SqlEditorPlugin;
use tracing::info;

#[derive(Parser, Debug)]
#[command(
    name = "tihc",
    version = "1.0.0",
    author = "Aric <ask.aric.su@gmail.com>",
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
    /// Whether to enable log cutting
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

fn register_all_plugins(
    kernel: &mut microkernel::platform::Microkernel,
    command_registry: &mut CommandRegistry,
) {
    let mut ctx = microkernel::plugin_api::traits::PluginContext {
        service_registry: kernel.service_registry.clone(),
        command_registry: Some(unsafe {
            std::mem::transmute::<&mut CommandRegistry, &'static mut CommandRegistry>(
                command_registry,
            )
        }),
        shutdown_rx: None, // Add this field; replace with actual receiver if needed
    };
    kernel
        .plugin_manager
        .register_plugin(Box::new(SlowLogPlugin::new()), &mut ctx);
    kernel
        .plugin_manager
        .register_plugin(Box::new(SqlEditorPlugin::new()), &mut ctx);
}

/// Main entry point for TiDB Intelligent Health Check (tihc) CLI/Web.
///
/// - CLI: tihc [OPTIONS] <SUBCOMMAND>
/// - Web: tihc server --host 127.0.0.1 --port 5000
#[tokio::main]
async fn main() -> Result<()> {
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

    // 打印通用欢迎信息到日志
    info!(target: "tihc", "🎯 Welcome to use TiDB Health Check (tihc) v1.1.0 starting");
    info!(target: "tihc", "📖 Documentation: https://www.askaric.com/en/tihc");
    info!(target: "tihc", "👨‍💻 Author: Aric <ask.aric.su@gmail.com>");

    let mut kernel = microkernel::platform::Microkernel::new(app_config.clone());
    let mut command_registry = CommandRegistry::new();
    register_all_plugins(&mut kernel, &mut command_registry);
    // === 演示 handler 层访问全局配置（只打印一次即可） ===
    {
        let _app_config = kernel.core_services.config_service.get();
        // info!(target: "tihc", "[demo] config.some_option={:?}", app_config.some_option);
    }
    // 优雅关闭信号监听，主流程等待 Ctrl+C 后主动退出
    let core_services = kernel.core_services.clone();
    match &cli.command {
        Some(Commands::Tools(tools_cmd)) => {
            let (cmd, args) = match tools_cmd {
                ToolsCommands::Slowlog(opts) => {
                    let mut args = Vec::new();
                    args.push(opts.log_dir.clone());
                    args.push(opts.pattern.clone());
                    
                    // 解析host中的端口信息
                    let (host, port) = if opts.host.contains(':') {
                        let parts: Vec<&str> = opts.host.split(':').collect();
                        if parts.len() == 2 {
                            (parts[0].to_string(), parts[1].parse::<u16>().unwrap_or(4000))
                        } else {
                            (opts.host.clone(), 4000)
                        }
                    } else {
                        (opts.host.clone(), 4000)
                    };
                    
                    // 手动构造数据库连接信息JSON字符串
                    let password_val = if opts.password.is_empty() { "null".to_string() } else { format!("\"{}\"", opts.password) };
                    let database_val = if opts.database == "tihc" { "null".to_string() } else { format!("\"{}\"", opts.database) };
                    
                    let conn_json = format!(
                        r#"{{"id":0,"name":"cli-connection","engine":"tidb","host":"{}","port":{},"username":"{}","password":{},"database":{},"use_tls":false,"ca_cert_path":null}}"#,
                        host, port, opts.user, password_val, database_val
                    );
                    args.push(conn_json);
                    
                    ("slowlog-import", args)
                }
            };
            
            info!(target: "tihc", "🚀 About to execute command: {} with {} args", cmd, args.len());
            for (i, arg) in args.iter().enumerate() {
                info!(target: "tihc", "  Arg[{}]: {}", i, if i == 2 { "[JSON Connection Data]" } else { arg });
            }
            
            let result = command_registry.execute(cmd, &args).await;
            match &result {
                Ok(value) => {
                    info!(target: "tihc", "✅ Command executed successfully: {}", value);
                    // 解析结果并打印到控制台
                    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(value.as_str().unwrap_or("{}")) {
                        if cmd == "slowlog-scan" {
                            if let Some(files) = json_value.get("matched_files").and_then(|f| f.as_array()) {
                                println!("📂 Found {} slow log file(s):", files.len());
                                for file in files {
                                    if let Some(file_path) = file.as_str() {
                                        println!("   📄 {}", file_path);
                                    }
                                }
                            }
                        } else if cmd == "slowlog-import" {
                            if let Some(imported_count) = json_value.get("imported_count") {
                                println!("✅ Successfully imported {} slow query records to database", imported_count);
                            }
                            if let Some(processed_files) = json_value.get("processed_files") {
                                println!("📊 Processed files: {}", processed_files);
                            }
                        }
                    }
                    println!("🎉 Slowlog operation completed successfully!");
                },
                Err(e) => {
                    info!(target: "tihc", "❌ Command execution failed: {}", e);
                    println!("❌ Error: {}", e);
                }
            }
            result?;
            // Tools 命令执行完成后直接退出，不需要等待信号
            info!(target: "tihc", "🎉 Tools command completed, exiting...");
        }
        Some(Commands::Server(web_opts)) => {
            let shutdown_rx = core_services.subscribe_shutdown();
            
            println!();
            println!("🎯 TiDB Health Check (tihc) Server");
            println!("==============================================");
            println!("🚀 Starting web server on {}:{}", web_opts.host, web_opts.port);
            println!("🌐 Server URL: http://{}:{}", web_opts.host, web_opts.port);
            println!("📝 Log Level: {}", cli.log_level);
            if !merged.log_file.is_empty() {
                let log_file_str = merged.log_file.as_ref();
                let log_path = std::path::Path::new(log_file_str);
                let absolute_path = if log_path.is_absolute() {
                    log_file_str.to_string()
                } else {
                    std::env::current_dir()
                        .unwrap_or_default()
                        .join(log_file_str)
                        .to_string_lossy()
                        .to_string()
                };
                println!("📁 Log File: {}", absolute_path);
            } else {
                println!("📁 Log File: console output (no file specified)");
            }
            // println!("⚙️  Config File: {}", cli.config_file);
            println!("==============================================");
            println!("✅ Server is ready to accept connections");
            println!();
            
            tracing::info!(target: "tihc", "Starting web server on {}:{}", web_opts.host, web_opts.port);

            commands::web::start_web_service(
                web_opts,
                command_registry,
                shutdown_rx,
            )
            .await?;
        }
        None => {
            Cli::command().print_help()?;
            println!();
        }
    }
    Ok(())
}
