// force include the backend plugin
#[allow(unused_imports)]
use plugin_backend as _;
use plugin_autoflow_client as _;
use plugin_ddl_checker as _;



// TiHC Microkernel Main Entry Point
use clap::{Args, CommandFactory, Parser, Subcommand};
use microkernel::PluginRegistry;
use microkernel::config::KernelConfig;
use microkernel::context::set_global_config;
use microkernel::log::init;
use microkernel::plugin::PluginFactory;
use microkernel::startup::run_axum_server;

fn init_logging(
    log_file: Option<&String>,
    log_level: &str,
    enable_rotation: bool,
) -> anyhow::Result<()> {
    if let Some(log_file) = log_file {
        init(log_file, log_level, enable_rotation)?;
    } else {
        tracing_subscriber::fmt()
            .with_max_level(match log_level {
                "error" => tracing::Level::ERROR,
                "warn" | "warning" => tracing::Level::WARN,
                "debug" => tracing::Level::DEBUG,
                "trace" => tracing::Level::TRACE,
                _ => tracing::Level::INFO,
            })
            .init();
    }
    Ok(())
}

fn merge_address_port(cli: &Cli, config: Option<&KernelConfig>) -> (String, u16) {
    match &cli.command {
        Some(Commands::Server(args)) => {
            let addr = if !args.listen.is_empty() {
                args.listen.clone()
            } else if let Some(cfg) = config {
                cfg.server.address.clone()
            } else {
                "127.0.0.1".to_string()
            };
            let port = if args.port != 8080 {
                args.port
            } else if let Some(cfg) = config {
                cfg.server.port
            } else {
                8080
            };
            (addr, port)
        }
        None => {
            if let Some(cfg) = config {
                (cfg.server.address.clone(), cfg.server.port)
            } else {
                ("127.0.0.1".to_string(), 8080)
            }
        }
    }
}

fn validate_socket_address(address: &str, port: u16) -> anyhow::Result<()> {
    let socket_addr = format!("{}:{}", address, port);
    if socket_addr.parse::<std::net::SocketAddr>().is_err() {
        anyhow::bail!("[FATAL] Invalid socket address: {}:{}", address, port);
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "tihc", version, about = "TiDB Healthy Checker")]
struct Cli {
    #[arg(
        short = 'l',
        long = "log-file",
        global = true,
        required = false,
        help = "Log file path"
    )]
    pub log_file: Option<String>,
    #[arg(
        short = 'L',
        long = "log-level",
        global = true,
        required = false,
        default_value = "info"
    )]
    pub log_level: String,
    #[arg(
        short = 'r',
        long = "log-rotation",
        global = true,
        required = false,
        default_value_t = false
    )]
    pub enable_log_rotation: bool,
    #[arg(
        short = 'c',
        long = "config",
        global = true,
        required = false,
        help = "Config file path"
    )]
    pub config: Option<String>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Start the HTTP server for dashboard and browser extensions integration")]
    Server(ServerOptions),
}

#[derive(Debug, Args, Default)]
pub struct ServerOptions {
    /// Listen address
    #[arg(long, default_value = "127.0.0.1")]
    pub listen: String,

    /// Listen port
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}

#[tokio::main]
/// TiHC 微内核主服务启动流程
/// 1. 解析 CLI 参数与配置文件
/// 2. 初始化全局配置与日志
/// 3. 检查服务启用状态
/// 4. 初始化事件总线与插件注册表，实现插件解耦
/// 5. 通过 inventory 自动注册所有插件，插件通过事件驱动注册路由
/// 6. 启动 axum 服务，所有 HTTP 路由统一分发到 PluginRegistry
async fn main() -> anyhow::Result<()> {
    // === 1. 解析 CLI 参数与配置文件 ===
    let cli = Cli::parse();
    if cli.command.is_none() {
        Cli::command().print_help()?;
        println!();
        return Ok(());
    }
    let config = cli
        .config
        .as_deref()
        .map(KernelConfig::from_file)
        .transpose()?
        .unwrap_or_else(|| KernelConfig::default());
    set_global_config(Arc::new(config.clone()));

    // === 2. 初始化日志 ===
    let default_log_file = microkernel::config::LogConfig::default().file.unwrap();
    let log_file: Option<&String> = cli
        .log_file
        .as_ref()
        .or_else(|| config.log.file.as_ref())
        .or(Some(&default_log_file));
    let log_level = if !cli.log_level.is_empty() {
        cli.log_level.as_str()
    } else {
        config.log.level.as_str()
    };
    let enable_log_rotation = cli.enable_log_rotation || config.log.enable_rotation;
    init_logging(log_file, log_level, enable_log_rotation)?;

    // === 3. 检查服务启用状态 ===
    if !config.server.enable {
        println!("[INFO] Server is disabled by config. Exiting.");
        return Ok(());
    }

    // === 4. 初始化事件总线与插件注册表 ===
    use microkernel::EventBus;
    use microkernel::plugin::PluginEvent;
    use std::sync::Arc;
    let bus = EventBus::<PluginEvent>::new(1024, 256);
    let plugin_registry = Arc::new(PluginRegistry::new());
    // 事件监听：用于插件路由注册、优雅停机等扩展
    let bus_rx = bus.subscribe();
    tokio::spawn(async move {
        let mut bus_rx = bus_rx;
        while let Ok(event) = bus_rx.recv().await {
            match event.payload {
                PluginEvent::RegisterHttpRoute(reg) => {
                    tracing::debug!(target: "microkernel", "[microkernel] Registered plugin HTTP route: {}", reg.path);
                }
                PluginEvent::GracefulShutdown => {
                    tracing::info!(target: "microkernel", "[microkernel] Received shutdown event");
                }
                PluginEvent::Custom(topic, value) => {
                    tracing::info!(target: "microkernel", "[microkernel] Custom event: topic={}, value={:?}", topic, value);
                }
            }
        }
    });

    // === 5. 自动注册所有插件（事件驱动） ===
    for factory in inventory::iter::<PluginFactory> {
        let plugin = (factory.0)();
        // 插件通过事件驱动注册路由，主服务无需关心具体业务
        plugin.register(bus.clone(), plugin_registry.clone());
    }

    // === 6. 启动 axum 服务，统一路由分发 ===
    let (address, port) = merge_address_port(&cli, Some(&config));
    println!("[INFO] tihc microkernel server starting...");
    println!("[INFO] Listen on http://{}:{}", address, port);
    if let Some(log_file) = log_file {
        println!("[INFO] Log file: {}", log_file);
    }
    validate_socket_address(&address, port)?;
    let result = run_axum_server(address, port, plugin_registry.clone(), Some(bus)).await;
    println!("[INFO] tihc microkernel server exited.");
    result
}
