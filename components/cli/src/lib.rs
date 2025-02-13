use clap::{Args, Subcommand};
use utils::{get_current_unix_time, get_thirty_minutes_ago};

// Options for docdb
#[derive(Args)]
pub struct DocdbOptions {
    #[clap(long, default_value = "127.0.0.1:12020", help = "Ng Monitor address")]
    pub ngurl: String,

    #[clap(long, default_value = "127.0.0.1:10080", help = "Instance address")]
    pub instance: String,

    // 使用函数返回值作为默认值
    #[clap(long, default_value_t = get_thirty_minutes_ago(), help = "Start time")]
    pub start: u64,

    #[clap(long, default_value_t = get_current_unix_time(), help = "End time")]
    pub end: u64,

    #[clap(long, default_value = "10000", help = "Top results")]
    pub top: u32,

    #[clap(long, default_value = "2s", help = "Time window")]
    pub window: String,
}

// Options for collect
#[derive(Args)]
pub struct CollectOptions {
    #[clap(long, default_value = "127.0.0.1:12020", help = "Ng Monitor address")]
    pub ngurl: String,

    #[clap(long, default_value = "127.0.0.1:10080", help = "Instance address")]
    pub instance: String,
}

// Subcommands for tools
#[derive(Subcommand)]
pub enum ToolsCommands {
    Docdb(DocdbOptions),
    Collect(CollectOptions),
}

// Options for chk
#[derive(Args)]
pub struct ChkOptions {
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
    Chk(ChkOptions),
}