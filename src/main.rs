use anyhow::{Context, Result};
use clap::Parser;
use cli::{commands::Command, Cli, Commands};
use std::time::Instant;
use tracing::level_filters::LevelFilter;
use utils::{cli_output::CommandOutput, log::init_logging};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_level = match cli.log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::TRACE,
        "debug" => LevelFilter::DEBUG,
        "info" => LevelFilter::INFO,
        "warn" => LevelFilter::WARN,
        "error" => LevelFilter::ERROR,
        _ => LevelFilter::INFO,
    };
    init_logging(&cli.log_file, log_level).with_context(|| "Failed to initialize logging")?;
    let start_time = Instant::now();
    let rt =
        tokio::runtime::Runtime::new().with_context(|| "Failed to initialize tokio runtime")?;
    match rt.block_on(cli.execute(start_time)) {
        Ok(output) => {
            output.display();
            Ok(())
        }
        Err(e) => {
            let error_chain: Vec<_> = e.chain().map(|e| format!("{}", e)).collect();
            tracing::error!(error = ?error_chain, "Command execution failed!");

            let command_name = match cli.command {
                Commands::Tools(_) => "tools",
                Commands::Collect(_) => "collect",
            };

            CommandOutput::new(command_name, "Failed", start_time.elapsed())
                .with_details(vec![("Errors", &error_chain.join("\n"))])
                .failed_display();

            std::process::exit(1);
        }
    }
}
