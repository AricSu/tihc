use anyhow::Result;
use std::path::Path;
use tracing_subscriber::fmt;

/// Initialize global logging using tracing/tracing-subscriber.
/// Supports file and console output, and log level.
pub fn init_logging(log_file: &str, log_level: &str, _enable_rotation: bool) -> Result<()> {
    let log_path = Path::new(log_file);
    let dir = log_path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(dir)?;
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    let level = match log_level.to_ascii_lowercase().as_str() {
        "error" => tracing::Level::ERROR,
        "warn" | "warning" => tracing::Level::WARN,
        "debug" => tracing::Level::DEBUG,
        "trace" => tracing::Level::TRACE,
        _ => tracing::Level::INFO,
    };
    fmt()
        .with_writer(file)
        .with_max_level(level)
        .with_ansi(false)
        .with_timer(tracing_subscriber::fmt::time())
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .init();
    Ok(())
}
