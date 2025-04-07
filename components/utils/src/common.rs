use crate::error::{Result, UtilsError};
use chrono::{TimeZone, Utc};
use std::path::Path;

pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir_all(path).map_err(|e| UtilsError::Io(e))?;
    }
    Ok(())
}

pub fn format_timestamp(ts: i64) -> Result<String> {
    Utc.timestamp_opt(ts, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .ok_or_else(|| UtilsError::Time("Invalid timestamp".into()))
}

pub fn generate_filename(prefix: &str, extension: &str) -> String {
    format!(
        "{}_{}{}",
        prefix,
        Utc::now().format("%Y%m%d_%H%M%S"),
        extension
    )
}

pub fn get_current_unix_time() -> u64 {
    Utc::now().timestamp() as u64
}

pub fn get_time_ago(minutes: u64) -> u64 {
    let seconds = minutes * 60;
    let current_time = get_current_unix_time();
    current_time.saturating_sub(seconds)
}

pub fn format_datetime(timestamp: i64) -> Option<String> {
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
}
