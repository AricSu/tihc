use anyhow::{Context, Result};
use clap::Args;
use serde_json::Value;
use utils::common::{get_current_unix_time, get_time_ago};

#[derive(Args)]
pub struct DocdbOptions {
    #[clap(long, default_value = "127.0.0.1:12020", help = "Ng Monitor address")]
    pub ngurl: String,

    #[clap(long, default_value = "127.0.0.1:10080", help = "Instance address")]
    pub instance: String,

    #[clap(long, default_value_t = get_time_ago(5), help = "Start time")]
    pub start: u64,

    #[clap(long, default_value_t = get_current_unix_time(), help = "End time")]
    pub end: u64,

    #[clap(long, default_value = "10000", help = "Top results")]
    pub top: u32,

    #[clap(long, default_value = "2s", help = "Time window")]
    pub window: String,

    #[clap(long, default_value = "topsql.json", help = "Limit")]
    pub storage: String,
}

impl DocdbOptions {
    pub async fn collect(&self) -> Result<String> {
        // Validate input parameters
        if self.start >= self.end {
            anyhow::bail!("Start time must be less than end time");
        }

        let url = format!(
            "http://{}/topsql/v1/sql_duration_count?end={}&instance={}&instance_type=tidb&start={}&top={}&window={}",
            self.ngurl,
            self.end,
            self.instance.replace(":", "%3A"),
            self.start,
            self.top,
            self.window,
        );

        // Send request with proper error context
        let response = reqwest::get(&url)
            .await
            .with_context(|| format!("Failed to connect to {}", self.ngurl))?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Server returned error {}: {}",
                response.status(),
                response.text().await?
            );
        }

        // Parse response
        let json: Value = response
            .json()
            .await
            .context("Failed to parse server response as JSON")?;

        // Save to file with error context
        std::fs::write(&self.storage, serde_json::to_string_pretty(&json)?)
            .with_context(|| format!("Failed to write data to {}", self.storage))?;

        Ok(self.storage.clone())
    }
}
