pub mod collect_pprof;
pub mod tools_docdb;

use clap::{Args, Parser, Subcommand};
use collect_pprof::DebugOptions;
use tools_docdb::DocdbOptions;

#[derive(Parser)]
#[clap(
    name = "tihc",
    version = "1.0",
    author = "Author: Aric",
    about = "TiHC CLI Tool\nEmail: askaric@gmail.com\nDoc: https://www.askaric.com/zh/"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(
        long,
        short = 'l',
        default_value = "tihc.log",
        global = true,
        help = "Log file path"
    )]
    pub log_file: String,

    #[clap(
        long,
        short = 'L',
        default_value = "info",
        global = true,
        help = "Log level (trace, debug, info, warn, error)"
    )]
    pub log_level: String,
}

// Subcommands for tools
#[derive(Subcommand)]
pub enum ToolsCommands {
    Docdb(DocdbOptions),
}

// Subcommands for collect
#[derive(Subcommand)]
pub enum CollectCommands {
    Debug(DebugOptions),
}

// Options for report
#[derive(Args)]
pub struct ReportOptions {
    #[clap(long, default_value = "127.0.0.1:12020", help = "Ng Monitor address")]
    pub ngurl: String,

    #[clap(long, default_value = "127.0.0.1:10080", help = "Instance address")]
    pub instance: String,
}

// Subcommands for tihc
#[derive(Subcommand)]
pub enum Commands {
    #[clap(subcommand)]
    Tools(ToolsCommands),
    Report(ReportOptions),
    #[clap(subcommand)]
    Collect(CollectCommands),
}
