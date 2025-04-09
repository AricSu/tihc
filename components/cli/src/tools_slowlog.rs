use anyhow::Context;
use clap::{Args, Parser};
use futures_util::stream::StreamExt;
use sqlx::MySqlPool;
use tokio::sync::mpsc;
use tools::{
    slow_log_retriever::{get_slowlog_files, SlowQueryRetriever},
    slow_query::DbOps,
};
use tracing::info;

#[derive(Parser)]
pub struct SlowlogCommand {
    #[clap(subcommand)]
    pub command: SlowlogCommands,
}

#[derive(Parser)]
pub enum SlowlogCommands {
    #[clap(about = "Parse TiDB slow log file and import to database")]
    Parse(SlowlogOptions),
}

#[derive(Args)]
pub struct SlowlogOptions {
    #[clap(
        long,
        short = 'a',
        help = "TiDB server address (e.g. 127.0.0.1:4000)",
        value_name = "HOST:PORT"
    )]
    pub host: String,

    #[clap(
        long,
        short = 'd',
        help = "Target database name for importing slow query data",
        default_value = "tihc"
    )]
    pub database: String,

    #[clap(long, short = 'u', help = "TiDB username", default_value = "root")]
    pub user: String,

    #[clap(long, short = 'p', help = "TiDB password", default_value = "")]
    pub password: String,

    #[clap(
        long,
        short = 'D',
        help = "Directory path containing TiDB slow query log files",
        value_name = "DIR"
    )]
    pub log_dir: String,

    #[clap(
        long,
        short = 'b',
        help = "Number of records to process in each batch",
        default_value = "64"
    )]
    pub batch_size: usize,

    #[clap(
        long,
        short = 't',
        help = "Slow query log filename pattern (e.g. \"tidb-slow*.log\")",
        value_name = "PATTERN"
    )]
    pub pattern: String,

    #[clap(
        long,
        help = "Timezone for parsing timestamps (e.g. UTC+8)",
        default_value = "UTC",
        value_parser = validate_timezone
    )]
    pub timezone: String,
}

impl SlowlogOptions {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        // Validate input parameters
        if self.batch_size == 0 {
            return Err(anyhow::anyhow!("Batch size must be greater than 0"));
        }

        // Get matching log files
        info!("Starting to scan log files in directory: {}", self.log_dir);
        let file_paths = get_slowlog_files(&self.log_dir, &self.pattern)
            .with_context(|| "Failed to get slow log files")?;

        if file_paths.is_empty() {
            info!("No matching log files found");
            return Ok(());
        }
        info!("Found {} matching log files", file_paths.len());

        // Initialize SlowQueryRetriever
        info!(
            "Initializing SlowQueryRetriever with batch size: {}",
            self.batch_size
        );
        let mut retriever = SlowQueryRetriever::new(self.batch_size, file_paths);

        // Create MySQL connection pool with timeout
        info!("Connecting to MySQL at: {}", self.host);
        let pool = MySqlPool::connect(&format!(
            "mysql://{}:{}@{}?connect_timeout=10",
            self.user, self.password, self.host
        ))
        .await
        .with_context(|| format!("Failed to connect to MySQL at {}", self.host))?;
        info!("Successfully connected to MySQL");

        // Initialize database and tables with proper error handling
        info!("Initializing database and tables");
        tools::slow_query::SlowQueryRow::init_db(&pool, &self.database)
            .await
            .with_context(|| "Failed to initialize database")?;
        tools::slow_query::SlowQueryRow::init_table(&pool, &self.database)
            .await
            .with_context(|| "Failed to create table")?;
        info!("Database and table initialization completed");

        // Create channel for processing results
        info!("Creating channel for processing results");
        let (sender, receiver) = mpsc::channel(1024);

        // Spawn consumer task with proper error handling
        let pool_clone = pool.clone();
        let timezone = self.timezone.clone();
        let database = self.database.clone();
        let consumer = tokio::spawn({
            let retriever = retriever.clone();
            async move {
                let mut count = 0;
                let mut batch_count = 0;
                let mut stream = Box::pin(retriever.data_for_slow_log(receiver).await);

                while let Some(result) = stream.next().await {
                    match result {
                        Ok(rows) => {
                            if rows.is_empty() {
                                continue;
                            }

                            batch_count += 1;
                            count += rows.len();
                            info!(
                                "Writing batch {} ({} rows) to database (total: {} rows)",
                                batch_count,
                                rows.len(),
                                count
                            );

                            tools::slow_query::SlowQueryRow::batch_insert(
                                &rows,
                                &pool_clone,
                                &database,
                                &timezone,
                            )
                            .await
                            .with_context(|| format!("Failed to insert batch {}", batch_count))?;

                            info!("Batch {} written successfully", batch_count);
                        }
                        Err(e) => {
                            return Err(anyhow::anyhow!("Error processing batch: {}", e));
                        }
                    }
                }

                info!(
                    "Successfully processed {} rows in {} batches",
                    count, batch_count
                );
                Ok::<(), anyhow::Error>(())
            }
        });

        // Process log files
        info!("Starting log file processing");
        retriever
            .parse_slow_log(sender)
            .await
            .with_context(|| "Failed to parse slow logs")?;

        // Wait for consumer task to complete
        consumer
            .await
            .with_context(|| "Consumer task failed to complete")?
            .with_context(|| "Consumer task encountered an error")?;

        // Close database connection
        pool.close().await;
        info!("Processing completed successfully");

        Ok(())
    }
}

fn validate_timezone(s: &str) -> Result<String, String> {
    let s = s.to_uppercase();
    if !s.starts_with("UTC") {
        return Err("Timezone must start with 'UTC'".to_string());
    }

    // 处理纯 UTC 的情况
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
