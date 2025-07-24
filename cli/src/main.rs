use anyhow::Result;
use clap::{Args, CommandFactory, Parser, Subcommand};

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
    /// Log file path
    #[arg(short = 'l', long = "log-file", global = true, default_value_t = default_log_file())]
    log_file: String,
    /// Log level
    #[arg(short = 'L', long = "log-level", global = true, default_value = "info")]
    log_level: String,
    #[command(subcommand)]
    command: Option<Commands>,
}

fn default_log_file() -> String {
    use chrono::Local;
    format!(
        "tihc_started_at_{}.log",
        Local::now().format("%Y%m%d_%H%M%S")
    )
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Inspection and diagnosis commands
    Check {
        #[command(subcommand)]
        check_cmd: CheckSubCommand,
    },
    /// Plugin related commands
    Plugin {
        #[command(subcommand)]
        plugin_cmd: PluginSubCommand,
    },
    /// Start web service
    Web {
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
}

#[derive(Subcommand, Debug)]
enum CheckSubCommand {
    /// Check DDL change risk
    LossyDdl {
        #[arg(long)]
        file: String,
    },
    /// Parse slowlog
    Slowlog {
        #[arg(long)]
        file: String,
    },
}

#[derive(Subcommand, Debug)]
enum PluginSubCommand {
    /// List all plugins
    List,
    /// Run a specific plugin
    Run {
        name: String,
        #[arg(long)]
        file: Option<String>,
    },
}

#[derive(Subcommand)]
enum CheckCommands {
    /// 检查 DDL 变更风险
    LossyDdl {
        #[arg(long)]
        file: String,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// 列出所有插件
    List,
    /// 运行指定插件
    Run {
        name: String,
        #[arg(long)]
        file: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // 全局参数可用于日志初始化等
    // println!("[TiHC] Log file: {} | Log level: {}", cli.log_file, cli.log_level);

    match &cli.command {
        Some(Commands::Check { check_cmd }) => match check_cmd {
            CheckSubCommand::LossyDdl { file } => {
                println!(
                    "[TiHC] DDL Risk Check\nInput file: {}\n[Mock Result] Check completed.",
                    file
                );
            }
            CheckSubCommand::Slowlog { file } => {
                println!(
                    "[TiHC] Slowlog Parse\nInput file: {}\n[Mock Result] Parse completed.",
                    file
                );
            }
        },
        Some(Commands::Plugin { plugin_cmd }) => match plugin_cmd {
            PluginSubCommand::List => {
                println!(
                    "[TiHC] Available plugins:\n- lossy_ddl        DDL Risk Check\n- slowlog_parser   Slowlog Parse"
                );
            }
            PluginSubCommand::Run { name, file } => {
                println!(
                    "[TiHC] Run plugin: {}\nInput file: {:?}\n[Mock Result] Execution completed.",
                    name, file
                );
            }
        },
        Some(Commands::Web { port }) => {
            println!(
                "[TiHC] Start web service, port: {}\n[Mock] Web service started.",
                port
            );
        }
        None => {
            Cli::command().print_help()?;
            println!();
        }
    }
    Ok(())
}
