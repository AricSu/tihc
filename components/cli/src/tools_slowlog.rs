use clap::{Args, Parser};

#[derive(Parser)]
pub struct SlowlogCommand {
    #[clap(subcommand)]
    pub command: SlowlogCommands,
}

#[derive(Parser)]
pub enum SlowlogCommands {
    #[clap(about = "Parse TiDB slow log file and import to database")]
    Format(FormatArgs),
}

#[derive(Args)]
pub struct FormatArgs {
    #[clap(long, short = 'a', help = "Database URL (e.g. localhost:4000)")]
    pub dbaddress: String,

    #[clap(
        long,
        short = 'D',
        value_name = "tihc",
        help = "database name import into"
    )]
    pub importdb: String,

    #[clap(long, short = 'u', help = "Database username")]
    pub dbusername: String,

    #[clap(long, short = 'p', help = "Database password")]
    pub dbpassword: String,

    #[clap(long, short = 'd', help = "Directory containing slow log files")]
    pub slowlogdir: String,

    #[clap(long, short = 'b', default_value = "64", help = "batch size")]
    pub batch: usize,

    #[clap(long, short = 'c', default_value = "4", help = "concurrency")]
    pub concurrency: usize,
}
