use std::fs::OpenOptions;
use std::path::Path;
use anyhow::Result;
use tracing_subscriber::fmt;

pub fn init(log_file: &str, log_level: &str, enable_log_rotation: bool) -> Result<()> {
    let log_path = Path::new(log_file);
    if log_file.trim().is_empty() {
        return Err(anyhow::anyhow!("Log file path is empty"));
    }
    if let Some(parent) = log_path.parent() {
        // If parent is empty (i.e. file in current dir), that's fine
        if parent.as_os_str().is_empty() {
            // Current directory, no need to create
        } else {
            std::fs::create_dir_all(parent)?;
        }
    }
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    if enable_log_rotation {
        eprintln!("[WARN] Log rotation is enabled, but not implemented. Logs will not rotate.");
    }
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
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .init();
    Ok(())
}

/// info 日志宏
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        ::tracing::info!($($arg)*);
    };
}

/// debug 日志宏
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        ::tracing::debug!($($arg)*);
    };
}

/// warn 日志宏
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        ::tracing::warn!($($arg)*);
    };
}

/// error 日志宏
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        ::tracing::error!($($arg)*);
    };
}

/// trace 日志宏
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        ::tracing::trace!($($arg)*);
    };
}