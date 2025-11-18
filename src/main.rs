use tower::util::ServiceExt;
use microkernel::PluginRegistry;
use axum::response::IntoResponse;
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

use clap::{Args, CommandFactory, Parser, Subcommand};
use microkernel::config::KernelConfig;
use microkernel::log::init;
use microkernel::run_axum_server;

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
async fn main() -> anyhow::Result<()> {
    // === 1. Parse CLI and config ===
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
        .transpose()?;

    // === 2. Init logging ===
    let default_log_file = microkernel::config::LogConfig::default().file.unwrap();
    let log_file: Option<&String> = cli.log_file.as_ref()
        .or_else(|| config.as_ref().and_then(|cfg| cfg.log.file.as_ref()))
        .or(Some(&default_log_file));
    let log_level = if !cli.log_level.is_empty() {
        cli.log_level.as_str()
    } else {
        config.as_ref().map(|c| c.log.level.as_str()).unwrap_or("info")
    };
    let enable_log_rotation = cli.enable_log_rotation
        || config.as_ref().map(|c| c.log.enable_rotation).unwrap_or(false);
    init_logging(log_file, log_level, enable_log_rotation)?;

    // === 3. Check server enable ===
    if let Some(cfg) = config.as_ref() {
        if !cfg.server.enable {
            println!("[INFO] Server is disabled by config. Exiting.");
            return Ok(());
        }
    }

    // === 4. Init plugin registry and register static handler ===
    let plugin_registry = std::sync::Arc::new(PluginRegistry::new());
    register_static_handler_via_eventbus(plugin_registry.clone()).await;

    // === 5. Build axum app and run server ===
    let plugin_router = Some(microkernel::plugin::plugin_router(plugin_registry.clone()));
    let (address, port) = merge_address_port(&cli, config.as_ref());
    println!("[INFO] tihc microkernel server starting...");
    println!("[INFO] Listen on http://{}:{}", address, port);
    if let Some(log_file) = log_file {
        println!("[INFO] Log file: {}", log_file);
    }
    validate_socket_address(&address, port)?;
    let result = run_axum_server(address, port, plugin_router).await;
    println!("[INFO] tihc microkernel server exited.");
    result
}

/// Register the static handler via EventBus before server starts
async fn register_static_handler_via_eventbus(plugin_registry: std::sync::Arc<PluginRegistry>) {
    use microkernel::plugin::{PluginEvent, RegisterHttpRoute};
    use backend::static_dist_router;
    use axum::extract::Request;
    use microkernel::{EventBus, EventEnvelope};
    let bus = EventBus::<PluginEvent>::new(1024, 256);
    let plugin_registry_clone = plugin_registry.clone();
    let static_router = static_dist_router();
    let handler: microkernel::plugin::PluginHandler = std::sync::Arc::new(
        move |req: Request<axum::body::Body>| {
            let router = static_router.clone();
            Box::pin(async move { router.oneshot(req).await.into_response() })
        },
    );
    let mut bus_rx = bus.subscribe();
    let reg_handle = tokio::spawn(async move {
        while let Ok(event) = bus_rx.recv().await {
            let PluginEvent::RegisterHttpRoute(reg) = event.payload;
            plugin_registry_clone.register_route(&reg.path, handler.clone());
            println!("[microkernel] Registered plugin HTTP route: {}", reg.path);
        }
    });
    let reg_event = EventEnvelope::new(
        "plugin_register_http_route",
        PluginEvent::RegisterHttpRoute(RegisterHttpRoute {
            path: "/".to_string(),
        }),
        None,
    );
    let _ = bus.broadcast(reg_event);
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    let _ = reg_handle.await;
}
