use crate::{Cli, CollectCommands, Commands, ToolsCommands};
use async_trait::async_trait;
use std::time::Instant;
use utils::cli_output::CommandOutput;

#[async_trait]
pub trait Command {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput, anyhow::Error>;
}

#[async_trait]
impl Command for Cli {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput, anyhow::Error> {
        match &self.command {
            Commands::Collect(cmd) => cmd.execute(start_time).await,
            Commands::Tools(cmd) => cmd.execute(start_time).await,
        }
    }
}

#[async_trait]
impl Command for CollectCommands {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput, anyhow::Error> {
        match self {
            CollectCommands::Pprof(debug_options) => debug_options.collect().await.map(|_| {
                CommandOutput::new("pprof", "Success", start_time.elapsed())
                    .with_details(vec![("Component", &debug_options.component)])
            }),
            CollectCommands::Docdb(docdb_options) => docdb_options.collect().await.map(|storage| {
                CommandOutput::new("docdb", "Success", start_time.elapsed())
                    .with_details(vec![("Storage", &storage)])
            }),
        }
    }
}

#[async_trait]
impl Command for ToolsCommands {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput, anyhow::Error> {
        match self {
            ToolsCommands::Slowlog(slowlog_options) => match slowlog_options.execute().await {
                Ok(_) => Ok(CommandOutput::new(
                    "slowlog",
                    "Success",
                    start_time.elapsed(),
                )),
                Err(e) => {
                    let error_message = format!("{}", e);
                    Ok(
                        CommandOutput::new("slowlog", "Failed", start_time.elapsed())
                            .with_details(vec![("Error", &error_message)]),
                    )
                }
            },
        }
    }
}
