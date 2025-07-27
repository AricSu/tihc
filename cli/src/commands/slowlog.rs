//! SlowlogOptions and related logic for slowlog batch import.
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct SlowlogOptions {
    /// TiDB server address (e.g. 127.0.0.1:4000).
    #[clap(long, short = 'a', value_name = "HOST:PORT")]
    pub host: String,
    /// Target database name for importing slow query data.
    #[clap(long, short = 'd', default_value = "tihc")]
    pub database: String,
    /// TiDB username.
    #[clap(long, short = 'u', default_value = "root")]
    pub user: String,
    /// TiDB password.
    #[clap(long, short = 'p', default_value = "")]
    pub password: String,
    /// Directory path containing TiDB slow query log files.
    #[clap(long, short = 'D', value_name = "DIR")]
    pub log_dir: String,
    /// Number of records to process in each batch.
    #[clap(long, short = 'b', default_value = "64")]
    pub batch_size: usize,
    /// Slow query log filename pattern (e.g. "tidb-slow*.log").
    #[clap(long, short = 't', value_name = "PATTERN")]
    pub pattern: String,
    /// Timezone for parsing timestamps (e.g. UTC+8).
    #[clap(long, default_value = "UTC")]
    pub timezone: String,
}

impl SlowlogOptions {
    pub fn validate_timezone(s: &str) -> Result<String, String> {
        let s = s.to_uppercase();
        if !s.starts_with("UTC") {
            return Err("Timezone must start with 'UTC'".to_string());
        }
        if s == "UTC" {
            return Ok(s);
        }
        let offset = &s[3..];
        if let Ok(hours) = offset.parse::<i32>() {
            if (-12..=14).contains(&hours) {
                Ok(s)
            } else {
                Err("Timezone offset must be between -12 and +14".to_string())
            }
        } else {
            Err("Invalid timezone format. Use format like 'UTC+8' or 'UTC-8'".to_string())
        }
    }
}
