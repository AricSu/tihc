
pub mod read_write;
pub mod sql_info;
use serde_json::Value;
use reqwest;
use std::{error::Error, io};
use tracing::{error, info};
use chrono::prelude::*;
use read_write::save_json_to_file;

pub fn get_current_unix_time() -> u64 {
    Utc::now().timestamp() as u64
}

pub fn get_thirty_minutes_ago() -> u64 {
    let thirty_minutes_in_seconds = 30 * 60;
    let current_time = get_current_unix_time();
    current_time - thirty_minutes_in_seconds
}
pub async fn fetch_and_save_json(
    ngurl: String,
    instance: String,
    start: u64,
    end: u64,
    top: u32,
    window: String,
    output_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "http://{}/topsql/v1/sql_duration_count?end={}&instance={}&instance_type=tidb&start={}&top={}&window={}",
        ngurl, end, instance, start, top, window
    );

    info!("Request URL: {}", &url);
    println!("Request URL: {}", &url);

    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        error!("Failed to fetch data: {}", response.status());
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to fetch data: {}", response.status()),
        )));
    }

    let mut json: Value = response.json().await?;

    remove_hash_fields(&mut json);

    save_json_to_file(&json, output_file_path)?;

    info!("Data successfully fetched and saved to {}", output_file_path);
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


