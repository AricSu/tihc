use crate::{slow_query::SlowQueryRow, slowlog_fields::SlowLogFields};
use anyhow::{Context, Result};
use regex::Regex;
use std::{
    fs,
    io::{self, BufRead},
};

pub fn split_by_colon(line: &str) -> (Vec<String>, Vec<String>) {
    fn is_letter_or_numeric(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }

    fn find_matched_right_bracket(line: &str, left_bracket_idx: usize) -> Option<usize> {
        let chars: Vec<char> = line.chars().collect();
        let left_bracket = chars[left_bracket_idx];
        let right_bracket = match left_bracket {
            '[' => ']',
            '{' => '}',
            _ => return None,
        };

        let mut current = left_bracket_idx;
        let mut left_bracket_cnt = 0;

        while current < chars.len() {
            let c = chars[current];
            if c == left_bracket {
                left_bracket_cnt += 1;
                current += 1;
            } else if c == right_bracket {
                left_bracket_cnt -= 1;
                if left_bracket_cnt > 0 {
                    current += 1;
                } else if left_bracket_cnt == 0 {
                    if current + 1 < chars.len() && !chars[current + 1].is_whitespace() {
                        return None;
                    }
                    return Some(current);
                } else {
                    return None;
                }
            } else {
                current += 1;
            }
        }
        None
    }

    let mut fields = Vec::new();
    let mut values = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut current = 0;
    let mut parse_key = true;
    let mut err_msg = None;

    while current < chars.len() {
        if parse_key {
            // Find the start position of the key
            while current < chars.len() && !is_letter_or_numeric(chars[current]) {
                current += 1;
            }
            let start = current;
            if current >= chars.len() {
                break;
            }

            // Find colon position
            while current < chars.len() && chars[current] != ':' {
                current += 1;
            }
            fields.push(line[start..current].trim().to_string());

            // Skip colon and space
            current += 2;
            if current >= chars.len() {
                values.push(String::new());
            }
            parse_key = false;
        } else {
            let start = current;
            if current < chars.len() && (chars[current] == '{' || chars[current] == '[') {
                if let Some(r_brace_idx) = find_matched_right_bracket(line, current) {
                    current = r_brace_idx + 1;
                } else {
                    err_msg = Some("Braces matched error");
                    break;
                }
            } else {
                while current < chars.len() && !chars[current].is_whitespace() {
                    current += 1;
                }
                // Handle empty value case: "Key: Key:"
                if current > 0 && chars[current - 1] == ':' {
                    values.push(String::new());
                    current = start;
                    parse_key = true;
                    continue;
                }
            }
            values.push(line[start..current].trim().to_string());
            parse_key = true;
        }
    }

    if let Some(msg) = err_msg {
        tracing::warn!("slow query parse slow log error: {}, Log: {}", msg, line);
        return (Vec::new(), Vec::new());
    }

    if fields.len() != values.len() {
        tracing::warn!(
            "slow query parse slow log error: field_count={}, value_count={}, Log: {}",
            fields.len(),
            values.len(),
            line
        );
        return (Vec::new(), Vec::new());
    }

    (fields, values)
}

fn parse_user_or_host_value(value: &str) -> String {
    // Handle formats like: root[root] or localhost [127.0.0.1]
    value.split('[').next().unwrap_or("").trim().to_string()
}

/// Set column value with type conversion and error handling
fn set_column_value(query: &mut SlowQueryRow, field: &str, value: &str, line_num: usize) {
    // Define an internal function to handle parsing errors and logging
    fn parse_with_log<T: std::str::FromStr + std::default::Default>(
        value: &str,
        field: &str,
        line_num: usize,
    ) -> T
    where
        T::Err: std::fmt::Display,
    {
        match value.parse::<T>() {
            Ok(val) => val,
            Err(e) => {
                tracing::warn!(
                    "Failed to parse slow query log field: field={}, value={}, line={}, error={}",
                    field,
                    value,
                    line_num,
                    e
                );
                T::default()
            }
        }
    }

    match field {
        // Integer types
        SlowLogFields::EXEC_RETRY_COUNT => {
            query.exec_retry_count = parse_with_log(value, field, line_num)
        }
        SlowLogFields::PREPROC_SUBQUERIES => {
            query.preproc_subqueries = parse_with_log(value, field, line_num)
        }
        SlowLogFields::REQUEST_COUNT => {
            query.request_count = parse_with_log(value, field, line_num)
        }
        SlowLogFields::TOTAL_KEYS => query.total_keys = parse_with_log(value, field, line_num),
        SlowLogFields::PROCESS_KEYS => query.process_keys = parse_with_log(value, field, line_num),
        SlowLogFields::ROCKSDB_DELETE_SKIPPED_COUNT => {
            query.rocksdb_delete_skipped_count = parse_with_log(value, field, line_num)
        }
        SlowLogFields::ROCKSDB_KEY_SKIPPED_COUNT => {
            query.rocksdb_key_skipped_count = value.parse().unwrap_or_default()
        }
        SlowLogFields::ROCKSDB_BLOCK_CACHE_HIT_COUNT => {
            query.rocksdb_block_cache_hit_count = value.parse().unwrap_or_default()
        }
        SlowLogFields::ROCKSDB_BLOCK_READ_COUNT => {
            query.rocksdb_block_read_count = value.parse().unwrap_or_default()
        }
        SlowLogFields::ROCKSDB_BLOCK_READ_BYTE => {
            query.rocksdb_block_read_byte = value.parse().unwrap_or_default()
        }
        SlowLogFields::TXN_RETRY => query.txn_retry = value.parse().unwrap_or_default(),
        SlowLogFields::WRITE_KEYS => query.write_keys = value.parse().unwrap_or_default(),
        SlowLogFields::WRITE_SIZE => query.write_size = value.parse().unwrap_or_default(),
        SlowLogFields::PREWRITE_REGION => query.prewrite_region = value.parse().unwrap_or_default(),
        SlowLogFields::RESULT_ROWS => query.result_rows = value.parse().unwrap_or_default(),
        SlowLogFields::MEM_MAX => query.mem_max = value.parse().unwrap_or_default(),
        SlowLogFields::DISK_MAX => query.disk_max = value.parse().unwrap_or_default(),
        SlowLogFields::CONN_ID => query.conn_id = value.parse().unwrap_or_default(),
        SlowLogFields::TXN_START_TS => query.txn_start_ts = value.parse().unwrap_or_default(),

        // 浮点数类型
        SlowLogFields::QUERY_TIME => query.query_time = value.parse().unwrap_or_default(),
        SlowLogFields::EXEC_RETRY_TIME => query.exec_retry_time = value.parse().unwrap_or_default(),
        SlowLogFields::PARSE_TIME => query.parse_time = value.parse().unwrap_or_default(),
        SlowLogFields::COMPILE_TIME => query.compile_time = value.parse().unwrap_or_default(),
        SlowLogFields::REWRITE_TIME => query.rewrite_time = value.parse().unwrap_or_default(),
        SlowLogFields::PREPROC_SUBQUERIES_TIME => {
            query.preproc_subqueries_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::OPTIMIZE_TIME => query.optimize_time = value.parse().unwrap_or_default(),
        SlowLogFields::WAIT_TS_TIME => query.wait_ts = value.parse().unwrap_or_default(),
        SlowLogFields::PREWRITE_TIME => query.prewrite_time = value.parse().unwrap_or_default(),
        SlowLogFields::WAIT_PREWRITE_BINLOG_TIME => {
            query.wait_prewrite_binlog_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::COMMIT_TIME => query.commit_time = value.parse().unwrap_or_default(),
        SlowLogFields::GET_COMMIT_TS_TIME => {
            query.get_commit_ts_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::COMMIT_BACKOFF_TIME => {
            query.commit_backoff_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::RESOLVE_LOCK_TIME => {
            query.resolve_lock_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::LOCAL_LATCH_WAIT_TIME => {
            query.local_latch_wait_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::COP_TIME => query.cop_time = value.parse().unwrap_or_default(),
        SlowLogFields::PROCESS_TIME => query.process_time = value.parse().unwrap_or_default(),
        SlowLogFields::WAIT_TIME => query.wait_time = value.parse().unwrap_or_default(),
        SlowLogFields::BACKOFF_TIME => query.backoff_time = value.parse().unwrap_or_default(),
        SlowLogFields::LOCK_KEYS_TIME => query.lock_keys_time = value.parse().unwrap_or_default(),
        SlowLogFields::WRITE_SQL_RESPONSE_TOTAL => {
            query.write_sql_response_total = value.parse().unwrap_or_default()
        }
        SlowLogFields::REQUEST_UNIT_READ => {
            query.request_unit_read = value.parse().unwrap_or_default()
        }
        SlowLogFields::REQUEST_UNIT_WRITE => {
            query.request_unit_write = value.parse().unwrap_or_default()
        }
        SlowLogFields::TIME_QUEUED_BY_RC => {
            query.time_queued_by_rc = value.parse().unwrap_or_default()
        }
        SlowLogFields::KV_TOTAL => query.kv_total = value.parse().unwrap_or_default(),
        SlowLogFields::PD_TOTAL => query.pd_total = value.parse().unwrap_or_default(),
        SlowLogFields::BACKOFF_TOTAL => query.backoff_total = value.parse().unwrap_or_default(),
        SlowLogFields::COP_PROC_AVG => query.cop_proc_avg = value.parse().unwrap_or_default(),
        SlowLogFields::COP_PROC_P90 => query.cop_proc_p90 = value.parse().unwrap_or_default(),
        SlowLogFields::COP_PROC_MAX => query.cop_proc_max = value.parse().unwrap_or_default(),
        SlowLogFields::COP_WAIT_AVG => query.cop_wait_avg = value.parse().unwrap_or_default(),
        SlowLogFields::COP_WAIT_P90 => query.cop_wait_p90 = value.parse().unwrap_or_default(),
        SlowLogFields::COP_WAIT_MAX => query.cop_wait_max = value.parse().unwrap_or_default(),
        SlowLogFields::TIDB_CPU_USAGE_DURATION => {
            query.tidb_cpu_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::TIKV_CPU_USAGE_DURATION => {
            query.tikv_cpu_time = value.parse().unwrap_or_default()
        }

        // 布尔类型
        SlowLogFields::IS_INTERNAL => query.is_internal = value == "true",
        SlowLogFields::PREPARED => query.prepared = value == "true",
        SlowLogFields::SUCC => query.succ = value == "true",
        SlowLogFields::IS_EXPLICIT_TXN => query.is_explicit_txn = value == "true",
        SlowLogFields::IS_WRITE_CACHE_TABLE => query.is_write_cache_table = value == "true",
        SlowLogFields::PLAN_FROM_CACHE => query.plan_from_cache = value == "true",
        SlowLogFields::PLAN_FROM_BINDING => query.plan_from_binding = value == "true",
        SlowLogFields::HAS_MORE_RESULTS => query.has_more_results = value == "true",

        // 字符串类型
        SlowLogFields::DIGEST => query.digest = value.to_string(),
        SlowLogFields::STATS_INFO => query.stats = value.to_string(),
        SlowLogFields::INDEX_NAMES => query.index_names = value.to_string(),
        SlowLogFields::QUERY => query.query = value.to_string(),
        SlowLogFields::PLAN => query.plan = value.to_string(),
        SlowLogFields::PLAN_DIGEST => query.plan_digest = value.to_string(),
        SlowLogFields::PREV_STMT => query.prev_stmt = value.to_string(),
        SlowLogFields::USER => query.user = value.to_string(),
        SlowLogFields::HOST => query.host = value.to_string(),
        SlowLogFields::SESSION_ALIAS => query.session_alias = value.to_string(),
        SlowLogFields::DB => query.db = value.to_string(),
        SlowLogFields::RESOURCE_GROUP => query.resource_group = value.to_string(),
        SlowLogFields::BACKOFF_TYPES => query.backoff_types = value.to_string(),
        SlowLogFields::COP_PROC_ADDR => query.cop_proc_addr = value.to_string(),
        SlowLogFields::COP_WAIT_ADDR => query.cop_wait_addr = value.to_string(),
        SlowLogFields::BINARY_PLAN => query.binary_plan = value.to_string(),
        SlowLogFields::BACKOFF_DETAIL => query.backoff_detail = value.to_string(),
        _ => {}
    }
}

pub fn parse_log(logs: &[Vec<String>]) -> io::Result<Vec<SlowQueryRow>> {
    let mut data = Vec::new();

    for log in logs {
        let mut query = SlowQueryRow::default();
        let mut start_flag = false;

        for (index, line) in log.iter().enumerate() {
            let line_num = index + 1;

            // Parse start marker line
            if line.starts_with(SlowLogFields::START_PREFIX) {
                query = SlowQueryRow::default();
                query.time = line[SlowLogFields::START_PREFIX.len()..].trim().to_string();
                start_flag = true;
                continue;
            }

            if !start_flag {
                continue;
            }

            // Process lines starting with ROW_PREFIX
            if line.starts_with(SlowLogFields::ROW_PREFIX) {
                let content = &line[SlowLogFields::ROW_PREFIX.len()..];

                // Handle special field branches
                if content.starts_with(SlowLogFields::PREV_STMT_PREFIX) {
                    set_column_value(
                        &mut query,
                        SlowLogFields::PREV_STMT,
                        &content[SlowLogFields::PREV_STMT_PREFIX.len()..],
                        line_num,
                    );
                } else if content.starts_with(SlowLogFields::USER_AND_HOST) {
                    let value = content
                        [SlowLogFields::USER_AND_HOST.len() + SlowLogFields::SPACE_MARK.len()..]
                        .trim();
                    if let Some((user_val, host)) = value.split_once('@') {
                        let user = parse_user_or_host_value(user_val);
                        set_column_value(&mut query, SlowLogFields::USER, &user, line_num);
                        let host = parse_user_or_host_value(host);
                        set_column_value(&mut query, SlowLogFields::HOST, &host, line_num);
                    }
                } else if content.starts_with(SlowLogFields::COP_BACKOFF_PREFIX) {
                    let new_detail = content.to_string();
                    if !query.backoff_detail.is_empty() {
                        query.backoff_detail.push_str(" ");
                        query.backoff_detail.push_str(&new_detail);
                    } else {
                        query.backoff_detail = new_detail;
                    }
                } else if content.starts_with(SlowLogFields::WARNINGS) {
                    set_column_value(
                        &mut query,
                        SlowLogFields::WARNINGS,
                        &content[SlowLogFields::WARNINGS.len() + SlowLogFields::SPACE_MARK.len()..],
                        line_num,
                    );
                } else if content.starts_with(SlowLogFields::DB) {
                    set_column_value(
                        &mut query,
                        SlowLogFields::DB,
                        &content[SlowLogFields::DB.len() + SlowLogFields::SPACE_MARK.len()..],
                        line_num,
                    );
                } else {
                    // Parse general fields
                    let (fields, values) = split_by_colon(content);
                    for (field, value) in fields.iter().zip(values.iter()) {
                        set_column_value(&mut query, field, value, line_num);
                    }
                }
            }
            // Process SQL end marker
            else if line.ends_with(SlowLogFields::SQL_SUFFIX) {
                // Skip 'use' statements
                if line.starts_with("use") {
                    continue;
                }

                // Set the final query statement
                set_column_value(&mut query, SlowLogFields::QUERY, line, line_num);

                // Add complete query to result set
                data.push(query);

                // Reset state
                query = SlowQueryRow::default();
                start_flag = false;
            } else {
                start_flag = false;
            }
        }
    }

    Ok(data)
}

/// Main structure for retrieving and processing slow query logs
#[derive(Debug, Clone, Default)]
pub struct SlowQueryRetriever {
    /// Number of logs to process in each batch
    pub batch_size: usize,
    /// Index of the current file being processed
    pub current_file_index: usize,
    /// List of file paths to process
    pub file_paths: Vec<String>,
    /// Current line number in the file
    pub file_line: usize,
}

impl SlowQueryRetriever {
    pub fn new(batch_size: usize, file_paths: Vec<String>) -> Self {
        let batch_size = batch_size.clamp(32, 256);
        tracing::debug!(
            "Initializing SlowQueryRetriever with batch_size: {}",
            batch_size
        );
        Self {
            batch_size,
            current_file_index: 0,
            file_paths,
            file_line: 0,
        }
    }

    /// Gets a file reader for the file at the specified index
    ///
    /// Returns None if the index is out of bounds
    async fn get_reader_at_index(&self, index: usize) -> Result<Option<io::BufReader<fs::File>>> {
        if index >= self.file_paths.len() {
            return Ok(None);
        }

        let file_path = &self.file_paths[index];
        tracing::debug!("Opening file: {}", file_path);
        let file = fs::File::open(file_path)
            .with_context(|| format!("Failed to open file: {}", file_path))?;
        Ok(Some(io::BufReader::with_capacity(64 * 1024, file))) // Use larger buffer for better performance
    }

    /// Gets a reader for the next file in the sequence
    ///
    /// Increments the current file index and attempts to get a reader for that file
    async fn get_next_reader(&mut self) -> Result<Option<io::BufReader<fs::File>>> {
        self.current_file_index += 1;
        tracing::debug!("Moving to next file, index: {}", self.current_file_index);
        self.get_reader_at_index(self.current_file_index).await
    }

    /// Retrieves a batch of log data from the current reader
    ///
    /// Processes up to `log_num` complete log entries, handling file transitions as needed
    pub async fn get_batch_log(
        &mut self,
        reader: &mut io::BufReader<fs::File>,
        offset: &mut usize,
        log_num: usize,
    ) -> Result<Vec<Vec<String>>> {
        let mut logs = Vec::with_capacity(log_num);
        let mut current_log = Vec::with_capacity(32); // Pre-allocate for typical log size

        for batch_idx in 0..log_num {
            loop {
                self.file_line += 1;
                let mut buffer = String::with_capacity(512); // Pre-allocate for typical line size

                let bytes_read = reader.read_line(&mut buffer).with_context(|| {
                    format!(
                        "Failed to read line {} in batch {}",
                        self.file_line, batch_idx
                    )
                })?;

                if bytes_read == 0 {
                    self.file_line = 0;
                    match self.get_next_reader().await? {
                        Some(new_reader) => {
                            *reader = new_reader;
                            tracing::debug!("Switched to next file at batch {}", batch_idx);
                            continue;
                        }
                        None => {
                            if !current_log.is_empty() {
                                logs.push(current_log);
                                tracing::debug!("Added final log at end of files");
                            }
                            return Ok(logs);
                        }
                    }
                }

                let line = buffer.trim_end().to_string();
                current_log.push(line.clone());

                // Check if this is the end of a SQL statement
                if line.ends_with(SlowLogFields::SQL_SUFFIX)
                    && !line.starts_with("use")
                    && !line.starts_with(SlowLogFields::ROW_PREFIX)
                {
                    break;
                }
            }

            logs.push(current_log);
            current_log = Vec::with_capacity(32);
            *offset += 1;
        }

        Ok(logs)
    }

    /// Parses slow query logs and sends the results through the provided channel
    ///
    /// Processes all files in the file_paths list, sending batches of parsed logs
    pub async fn parse_slow_log(
        &mut self,
        sender: tokio::sync::mpsc::Sender<Result<Vec<SlowQueryRow>>>,
    ) -> Result<()> {
        // Early return if no files to process
        if self.file_paths.is_empty() {
            tracing::info!("No slow query log files to process");
            return Ok(());
        }

        let mut offset = 0;
        self.current_file_index = 0;

        let mut reader = self
            .get_reader_at_index(0)
            .await?
            .with_context(|| "Failed to initialize first file reader")?;

        let mut total_processed = 0;
        let start_time = std::time::Instant::now();

        loop {
            match self
                .get_batch_log(&mut reader, &mut offset, self.batch_size)
                .await
            {
                Ok(batch) => {
                    if batch.is_empty() {
                        break;
                    }

                    total_processed += batch.len();
                    if total_processed % 1000 == 0 {
                        let elapsed = start_time.elapsed();
                        tracing::info!(
                            "Processed {} records in {:.2}s ({:.2} records/s)",
                            total_processed,
                            elapsed.as_secs_f64(),
                            total_processed as f64 / elapsed.as_secs_f64()
                        );
                    }

                    let parsed_result = parse_log(&batch)
                        .with_context(|| format!("Failed to parse batch at offset {}", offset))
                        .map_err(anyhow::Error::from);

                    sender
                        .send(parsed_result)
                        .await
                        .with_context(|| "Channel closed unexpectedly")?;
                }
                Err(e) => {
                    let error_msg = format!("Batch processing error at offset {}: {}", offset, e);
                    tracing::error!("{}", error_msg);
                    sender
                        .send(Err(anyhow::anyhow!(error_msg)))
                        .await
                        .with_context(|| "Failed to send error through channel")?;
                }
            }
        }

        let elapsed = start_time.elapsed();
        tracing::info!(
            "Processing completed: {} records in {:.2}s ({:.2} records/s)",
            total_processed,
            elapsed.as_secs_f64(),
            total_processed as f64 / elapsed.as_secs_f64()
        );

        Ok(())
    }

    /// Converts received data into an async stream of results
    ///
    /// Filters out empty results and transforms the receiver into a stream
    pub async fn data_for_slow_log(
        &self,
        receiver: tokio::sync::mpsc::Receiver<Result<Vec<SlowQueryRow>>>,
    ) -> impl futures::Stream<Item = Result<Vec<SlowQueryRow>>> + '_ {
        futures::stream::unfold(receiver, |mut receiver| async move {
            match receiver.recv().await {
                Some(Ok(rows)) if !rows.is_empty() => Some((Ok(rows), receiver)),
                Some(Err(e)) => Some((Err(e), receiver)),
                Some(Ok(_)) => None, // skip empty results
                None => None,        // channel closed
            }
        })
    }
}

/// Gets slow query log files that match the specified template
///
/// Returns a list of file paths that match the regex pattern in the specified directory
pub fn get_slowlog_files(slowlogdir: &str, logtemplate: &str) -> Result<Vec<String>> {
    // Compile the regex pattern
    let regex = Regex::new(logtemplate)
        .with_context(|| format!("Invalid regex pattern: {}", logtemplate))?;

    // Check if directory exists and is accessible
    let path = std::path::Path::new(slowlogdir);
    if !path.exists() {
        return Err(anyhow::anyhow!("Directory does not exist: {}", slowlogdir));
    }
    if !path.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {}", slowlogdir));
    }

    // Read directory entries
    let dir = fs::read_dir(slowlogdir)
        .with_context(|| format!("Failed to read directory: {}", slowlogdir))?;

    // Filter and collect matching files with improved error handling
    let mut files = Vec::new();
    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!("Failed to read directory entry: {}", e);
                continue;
            }
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => {
                tracing::warn!("Invalid filename encoding: {:?}", path);
                continue;
            }
        };

        if regex.is_match(file_name) {
            files.push(path.to_string_lossy().into_owned());
        }
    }

    // Sort files for consistent ordering
    files.sort();

    // Log the results
    match files.len() {
        0 => tracing::info!("No matching slow query log files found in {}", slowlogdir),
        1 => tracing::info!("Found 1 matching slow query log file"),
        n => tracing::info!("Found {} matching slow query log files", n),
    }

    // Log individual files at debug level
    for file in &files {
        tracing::debug!("Matched file: {}", file);
    }

    Ok(files)
}
