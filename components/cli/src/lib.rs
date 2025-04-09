pub mod collect_docdb;
pub mod collect_pprof;
pub mod commands;
pub mod tools_slowlog;

use clap::{Parser, Subcommand};
use collect_docdb::DocdbOptions;
use collect_pprof::PprofOptions;
use tools_slowlog::SlowlogOptions;

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

#[derive(Subcommand)]
pub enum Commands {
    #[clap(subcommand)]
    #[clap(about = "Collect nessary info from TiDB components")]
    Tools(ToolsCommands),

    #[clap(subcommand)]
    #[clap(about = "There are some commands for tuning and investigation")]
    Collect(CollectCommands),
}

#[derive(Subcommand)]
pub enum CollectCommands {
    #[clap(about = "Collect performance TOPSQL from Docdb of ng-monitor")]
    Docdb(DocdbOptions),
    #[clap(about = "Collect performance info from TiDB components (tidb, tikv, pd, br)")]
    Pprof(PprofOptions),
}

#[derive(Subcommand)]
pub enum ToolsCommands {
    #[clap(about = "Parse TiDB slow log file and import to database")]
    Slowlog(SlowlogOptions),
}
