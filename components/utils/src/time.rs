use chrono::{TimeZone, Utc};

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
