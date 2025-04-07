use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, CollectCommands, Commands};
use std::time::Instant;
use tracing::level_filters::LevelFilter;
use utils::{
    cli_output::CommandOutput,
    log::init_logging,
    sql_info::{extract_and_replace_sql_info, generate_html_from_sql_info},
};

#[async_trait::async_trait]
trait Command {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput>;
}

#[async_trait::async_trait]
impl Command for CollectCommands {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput> {
        match self {
            CollectCommands::Pprof(debug_options) => debug_options.collect().await.map(|_| {
                CommandOutput::new("pprof", "Success", start_time.elapsed())
                    .with_details(vec![("Component", &debug_options.component)])
            }),
            // CollectCommands::Docdb(docdb_options) => {
            //     let sql_infos = extract_and_replace_sql_info("topsql_data.json")
            //         .context("Failed to extract and replace SQL information")?;
            //     generate_html_from_sql_info(
            //         &sql_infos,
            //         "topsql_data_result.html",
            //         docdb_options.start,
            //         docdb_options.end,
            //     )
            //     .context("Failed to generate HTML report")?;

            //     Ok(
            //         CommandOutput::new("docdb", "Success", start_time.elapsed()).with_details(
            //             vec![
            //                 ("Instance", &docdb_options.instance),
            //                 ("Start Time", &docdb_options.start.to_string()),
            //                 ("End Time", &docdb_options.end.to_string()),
            //             ],
            //         ),
            //     )
            // }
        }
    }
}

#[async_trait::async_trait]
impl Command for Commands {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput> {
        match self {
            Commands::Collect(cmd) => cmd.execute(start_time).await,
            // Commands::Report(_) => anyhow::bail!("Report feature not implemented yet"),
            Commands::Tools(_) => anyhow::bail!("Tools feature not implemented yet"),
        }
    }
}

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
    match rt.block_on(cli.command.execute(start_time)) {
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
                // Commands::Report(_) => "report",
            };

            CommandOutput::new(command_name, "Failed", start_time.elapsed())
                .with_details(vec![("Errors", &error_chain.join("\n"))])
                .failed_display();

            std::process::exit(1);
        }
    }
}
