pub mod collect_docdb;
// Module for collecting performance profiles from TiDB components
pub mod collect_pprof;
pub mod tools_slowlog;

use clap::{Parser, Subcommand};
use collect_docdb::DocdbOptions;
use collect_pprof::PprofOptions;
use tools_slowlog::SlowlogCommand;

#[derive(Parser)]
#[clap(
    name = "tihc",
    version = "1.0.0",
    author = "Author: Aric",
    about = "TiHC CLI Tool\nEmail: askaric@gmail.com\nDoc: https://www.askaric.com/en/tihc"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(
        long,
        short = 'l',
        default_value_t = utils::log::generate_log_filename(),
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

// Subcommands for collect
#[derive(Subcommand)]
pub enum CollectCommands {
    #[clap(about = "Collect performance info from TiDB components (tidb, tikv, pd, br)")]
    Pprof(PprofOptions),
    // Docdb(DocdbOptions),
}

// Subcommands for tihc
#[derive(Subcommand)]
pub enum Commands {
    #[clap(subcommand)]
    Tools(ToolsCommands),
    // Report(ReportOptions),
    #[clap(subcommand)]
    #[clap(about = "Collect info from TiDB components (tidb, tikv, pd, br)")]
    Collect(CollectCommands),
}

// Subcommands for tools
#[derive(Subcommand)]
pub enum ToolsCommands {
    #[clap(about = "Docdb tools")]
    Docdb(DocdbOptions),
    #[clap(about = "Format TiDB slow log")]
    Slowlog(SlowlogCommand),
}
