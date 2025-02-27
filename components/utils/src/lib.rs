pub mod cli_output;
pub mod error;
pub mod log;
pub mod process;
pub mod profile;
pub mod read_write;
pub mod sql_info;
pub mod time;

use anyhow::{bail, Context};
pub use error::Result;
use read_write::save_json_to_file;
use reqwest;
use serde_json::Value;
use tracing::{error, info};

pub async fn fetch_and_save_json(
    ngurl: String,
    instance: String,
    start: u64,
    end: u64,
    top: u32,
    window: String,
    output_file_path: &str,
) -> Result<()> {
    let url = format!(
        "http://{}/topsql/v1/sql_duration_count?end={}&instance={}&instance_type=tidb&start={}&top={}&window={}",
        ngurl, end, instance, start, top, window
    );

    info!("Request URL: {}", &url);

    let response = reqwest::get(&url).await.context("Failed to fetch data")?;

    if !response.status().is_success() {
        error!("Failed to fetch data: {}", response.status());
        bail!("Failed to fetch data: {}", response.status());
    }

    let mut json: Value = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    remove_hash_fields(&mut json);

    save_json_to_file(&json, output_file_path).context("Failed to save JSON to file")?;

    info!(
        "Data successfully fetched and saved to {}",
        output_file_path
    );
    Ok(())
}

fn remove_hash_fields(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.remove("hash");
            for v in map.values_mut() {
                remove_hash_fields(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                remove_hash_fields(v);
            }
        }
        _ => {}
    }
}
