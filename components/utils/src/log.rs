use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::{Arc, Mutex};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};

// Wrapper struct that implements the Write trait
struct SharedWriter(Arc<Mutex<BufWriter<File>>>);

impl Write for SharedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut writer = self.0.lock().expect("Failed to lock the writer");
        let result = writer.write(buf);
        writer.flush()?; // 每次写入后立即刷新
        result
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut writer = self.0.lock().expect("Failed to lock the writer");
        writer.flush()
    }
}

/// Initialize the logging system, outputting logs to both the console and a file
///
/// # Arguments
/// - `log_file_path`: The path to the log file
/// - `log_level`: The logging level, e.g., `LevelFilter::INFO`
///
/// # Returns
/// If initialization is successful, returns `Ok(())`; otherwise, returns an error message
pub fn init_logging(log_file_path: &str, log_level: LevelFilter) -> Result<(), std::io::Error> {
    // 打开日志文件
    let file = File::create(log_file_path)?;
    let file_writer = Arc::new(Mutex::new(BufWriter::new(file)));

    // 文件日志层：包含所有指定级别及以上的日志
    let file_layer = layer()
        .with_writer(move || SharedWriter(file_writer.clone()))
        .with_ansi(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_filter(log_level);

    // 创建订阅者，只包含文件日志层
    let subscriber = Registry::default().with(file_layer);

    // 设置全局默认订阅者
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(())
}
