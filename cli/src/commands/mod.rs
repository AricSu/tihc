pub mod precheck;
pub mod slowlog;
pub mod web;

use anyhow::Result;
use clap::{Parser, Subcommand};
use microkernel::platform::message_bus;
use slowlog::SlowlogOptions;
use web::WebOptions;

#[derive(Parser, Debug)]
#[command(
    name = "tihc",
    version = env!("CARGO_PKG_VERSION"),
    author = "Aric <ask.aric.su@gmail.com>",
    about = "TiDB Intelligent Health Check (tihc) CLI Tool",
    long_about = "A CLI for TiDB Intelligent Health Check (tihc)\nDoc: https://www.askaric.com/en/tihc",
    after_help = "USAGE:\n    tihc [OPTIONS] <SUBCOMMAND>\n\nFor more information, visit: https://www.askaric.com/en/tihc"
)]
pub struct Cli {
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
        long = "enable-log-rotation",
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
        default_value = "config.toml",
        help = "Config file path"
    )]
    pub config_file: String,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(subcommand, about = "Collect nessary info from TiDB components")]
    Tools(ToolsCommands),
    #[clap(about = "Start the HTTP server for dashboard integration")]
    Server(WebOptions),
}

#[derive(Subcommand, Debug)]
pub enum ToolsCommands {
    // #[clap(about = "Parse TiDB slow log file and import to database")]
    // Slowlog(SlowlogOptions),
    #[clap(about = "Check DDL SQL for lossy operations")]
    Ddlcheck(precheck::DDLCheckOptions),
}

pub async fn handle_tools_command(tools_cmd: &ToolsCommands) -> Result<()> {
    match tools_cmd {
        // ToolsCommands::Slowlog(opts) => slowlog::handle_slowlog(opts).await,
        ToolsCommands::Ddlcheck(opts) => precheck::handle_ddlcheck(opts).await,
    }
}

pub async fn handle_server_command(web_opts: WebOptions) -> Result<()> {
    web::start_web_service(web_opts).await
}
