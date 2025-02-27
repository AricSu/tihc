use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, CollectCommands, Commands, ToolsCommands};
use std::time::Instant;
use tracing::level_filters::LevelFilter;
use utils::{
    cli_output::CommandOutput,
    fetch_and_save_json,
    log::init_logging,
    sql_info::{extract_and_replace_sql_info, generate_html_from_sql_info},
};

#[async_trait::async_trait]
trait Command {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput>;
}

#[async_trait::async_trait]
impl Command for ToolsCommands {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput> {
        match self {
            ToolsCommands::Docdb(options) => {
                fetch_and_save_json(
                    options.ngurl.clone(),
                    options.instance.clone(),
                    options.start,
                    options.end,
                    options.top,
                    options.window.clone(),
                    "topsql_data.json",
                )
                .await
                .context("获取和保存 JSON 数据失败")?;

                let sql_infos = extract_and_replace_sql_info("topsql_data.json")
                    .context("提取和替换 SQL 信息失败")?;
                generate_html_from_sql_info(
                    &sql_infos,
                    "topsql_data_result.html",
                    options.start,
                    options.end,
                )
                .context("生成 HTML 报告失败")?;

                Ok(
                    CommandOutput::new("docdb", "Success", start_time.elapsed()).with_details(
                        vec![
                            ("Instance", &options.instance),
                            ("Start Time", &options.start.to_string()),
                            ("End Time", &options.end.to_string()),
                        ],
                    ),
                )
            }
        }
    }
}

#[async_trait::async_trait]
impl Command for CollectCommands {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput> {
        match self {
            CollectCommands::Debug(debug_options) => debug_options.collect().await.map(|_| {
                CommandOutput::new("debug", "Success", start_time.elapsed())
                    .with_details(vec![("Component", &debug_options.component)])
            }),
        }
    }
}

#[async_trait::async_trait]
impl Command for Commands {
    async fn execute(&self, start_time: Instant) -> Result<CommandOutput> {
        match self {
            Commands::Tools(cmd) => cmd.execute(start_time).await,
            Commands::Collect(cmd) => cmd.execute(start_time).await,
            Commands::Report(_) => anyhow::bail!("报告功能暂未实现"),
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
    init_logging(&cli.log_file, log_level).with_context(|| "日志初始化失败")?;
    let start_time = Instant::now();
    let rt = tokio::runtime::Runtime::new().with_context(|| "创建 tokio runtime 失败")?;
    match rt.block_on(cli.command.execute(start_time)) {
        Ok(output) => {
            output.display();
            Ok(())
        }
        Err(e) => {
            let error_chain: Vec<_> = e.chain().map(|e| format!("{}", e)).collect();
            tracing::error!(error = ?error_chain, "命令执行失败");

            let command_name = match cli.command {
                Commands::Tools(_) => "tools",
                Commands::Collect(_) => "collect",
                Commands::Report(_) => "report",
            };

            CommandOutput::new(command_name, "Failed", start_time.elapsed())
                .with_details(vec![("Errors", &error_chain.join("\n"))])
                .failed_display();

            std::process::exit(1);
        }
    }
}
