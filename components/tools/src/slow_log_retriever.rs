use crate::{slow_query::SlowQueryRow, slowlog_fields::SlowLogFields};
use std::io::{self, BufRead};
use std::sync::atomic::{AtomicUsize, Ordering};
use chrono::DateTime;
use threadpool::ThreadPool;


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
            // 查找键的开始位置
            while current < chars.len() && !is_letter_or_numeric(chars[current]) {
                current += 1;
            }
            let start = current;
            if current >= chars.len() {
                break;
            }

            // 查找冒号位置
            while current < chars.len() && chars[current] != ':' {
                current += 1;
            }
            fields.push(line[start..current].trim().to_string());

            // 跳过冒号和空格
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
                // 处理空值情况: "Key: Key:"
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
    // 处理格式如: root[root] 或 localhost [127.0.0.1]
    value.split('[').next().unwrap_or("").trim().to_string()
}

pub fn parse_time(time_str: &str) -> String {
    if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
        return dt.format("%Y-%m-%d %H:%M:%S%.6f").to_string();
    }
    // 如果解析失败，记录警告并返回原始字符串
    tracing::warn!("Failed to parse time string: {}", time_str);
    time_str.to_string()
}


/// 新增通用字段设置方法
fn set_column_value(query: &mut SlowQueryRow, field: &str, value: &str) {
    match field {
        // 整数类型
        SlowLogFields::EXEC_RETRY_COUNT => {
            query.exec_retry_count = value.parse().unwrap_or_default()
        }
        SlowLogFields::PREPROC_SUBQUERIES => {
            query.preproc_subqueries = value.parse().unwrap_or_default()
        }
        SlowLogFields::REQUEST_COUNT => query.request_count = value.parse().unwrap_or_default(),
        SlowLogFields::TOTAL_KEYS => query.total_keys = value.parse().unwrap_or_default(),
        SlowLogFields::PROCESS_KEYS => query.process_keys = value.parse().unwrap_or_default(),
        SlowLogFields::ROCKSDB_DELETE_SKIPPED_COUNT => {
            query.rocksdb_delete_skipped_count = value.parse().unwrap_or_default()
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

        // 其他特殊处理
        _ => {}
    }
}


pub fn parse_log(logs: &[Vec<String>]) -> io::Result<Vec<SlowQueryRow>> {
    let mut data = Vec::new();

    for log in logs {
        let mut query = SlowQueryRow::default();
        let mut start_flag = false;

        for line in log {
            // 解析开始标记行
            if line.starts_with(SlowLogFields::START_PREFIX) {
                query = SlowQueryRow::default();
                // query.time = parse_time(&line[SlowLogFields::START_PREFIX.len()..].trim());
                query.time = line[SlowLogFields::START_PREFIX.len()..].trim().to_string();
                start_flag = true;
                continue;
            }

            if !start_flag {
                continue;
            }

            // 处理 ROW_PREFIX 开头的行
            if line.starts_with(SlowLogFields::ROW_PREFIX) {
                let content = &line[SlowLogFields::ROW_PREFIX.len()..];

                // 处理特殊字段分支
                if content.starts_with(SlowLogFields::PREV_STMT_PREFIX) {
                    set_column_value(
                        &mut query,
                        SlowLogFields::PREV_STMT,
                        &content[SlowLogFields::PREV_STMT_PREFIX.len()..],
                    );
                } else if content.starts_with(SlowLogFields::USER_AND_HOST) {
                    let value = content
                        [SlowLogFields::USER_AND_HOST.len() + SlowLogFields::SPACE_MARK.len()..]
                        .trim();
                    if let Some((user_val, host)) = value.split_once('@') {
                        let user = parse_user_or_host_value(user_val);
                        set_column_value(&mut query, SlowLogFields::USER, &user);
                        let host = parse_user_or_host_value(host);
                        set_column_value(&mut query, SlowLogFields::HOST, &host);
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
                    );
                } else if content.starts_with(SlowLogFields::DB) {
                    set_column_value(
                        &mut query,
                        SlowLogFields::DB,
                        &content[SlowLogFields::DB.len() + SlowLogFields::SPACE_MARK.len()..],
                    );
                } else {
                    // 通用字段解析
                    let (fields, values) = split_by_colon(content);
                    for (field, value) in fields.iter().zip(values.iter()) {
                        set_column_value(&mut query, field, value);
                    }
                }
            }
            // 处理 SQL 结束标记
            else if line.ends_with(SlowLogFields::SQL_SUFFIX) {
                // 忽略 use 语句
                if line.starts_with("use") {
                    continue;
                }

                // 设置最终查询语句
                query.query = line.to_string();

                // 添加完整查询到结果集
                data.push(query);

                // 重置状态
                query = SlowQueryRow::default();
                start_flag = false;
            } else {
                start_flag = false;
            }
        }
    }
    Ok(data)
}

pub fn get_batch_log<R: BufRead>(
    reader: &mut R,
    offset: &mut Offset,
    batch_size: usize
) -> io::Result<Vec<Vec<String>>> {
    let mut logs = Vec::new();
    let mut current_log = Vec::new();
    let mut buffer = String::with_capacity(batch_size);

    // 跳过已读取的行
    for _ in 0..offset.offset {
        if reader.read_line(&mut buffer)? == 0 {
            return Ok(logs);
        }
        buffer.clear();
    }

    while reader.read_line(&mut buffer)? > 0 {
        let line = buffer.trim_end();

        if line.starts_with(SlowLogFields::START_PREFIX) && !current_log.is_empty() {
            logs.push(current_log);
            current_log = Vec::new();
        }

        current_log.push(line.to_string());
        buffer.clear();
    }

    // 添加最后一个日志
    if !current_log.is_empty() {
        logs.push(current_log);
    }

    Ok(logs)
}


// 在文件顶部添加以下导入
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// 修改 Offset 结构体，实现 Default trait
#[derive(Debug, Clone, Default)]
pub struct Offset {
    offset: usize,
    length: usize,
}

pub struct SlowQueryRetriever {
    max_concurrency: usize, // 最大并发数
    batch_size: usize, // 每次读取的日志数量
}



impl SlowQueryRetriever {
    pub fn new(max_concurrency: usize, batch_size: usize) -> Self {
        Self { max_concurrency, batch_size }
    }

    pub fn parse_slow_log_concurrent<R: BufRead + Send + 'static>(
        &self,
        reader: R,
        concurrency: usize,
        batch_size: usize,
    ) -> io::Result<Vec<SlowQueryRow>> {
        let (tx, rx) = mpsc::sync_channel(concurrency * 2); // 使用同步通道提高性能
        let reader = Arc::new(Mutex::new(reader));
        let active_tasks = Arc::new(AtomicUsize::new(0)); // 使用原子类型替代Mutex
        let mut offset = Offset::default();

        let handle = thread::spawn(move || -> io::Result<()> {
            Self::process_logs(reader, active_tasks, tx, concurrency, batch_size, &mut offset)
        });

        let results = Self::collect_results(rx)?;
        handle.join().map_err(|_| io::Error::new(io::ErrorKind::Other, "Thread panicked"))??;
        
        Ok(results)
    }

    fn process_logs(
        reader: Arc<Mutex<impl BufRead>>,
        active_tasks: Arc<AtomicUsize>,
        tx: mpsc::SyncSender<io::Result<Vec<SlowQueryRow>>>,
        concurrency: usize,
        batch_size: usize,
        offset: &mut Offset,
    ) -> io::Result<()> {
        let thread_pool = ThreadPool::new(concurrency);

        loop {
            while active_tasks.load(Ordering::SeqCst) >= concurrency {
                thread::sleep(Duration::from_millis(5));
            }

            let batch = {
                let mut locked_reader = reader.lock().unwrap();
                let batch = get_batch_log(&mut *locked_reader, offset, batch_size)?;
                
                // 更新offset
                offset.offset += batch.iter().map(|log| log.len()).sum::<usize>();
                offset.length = batch.last().map_or(0, |log| log.len());
                
                batch
            };

            if batch.is_empty() {
                break;
            }

            active_tasks.fetch_add(1, Ordering::SeqCst);
            let tx = tx.clone();
            let active_tasks = active_tasks.clone();

            thread_pool.execute(move || {
                let result = parse_log(&batch);
                tx.send(result).unwrap();
                active_tasks.fetch_sub(1, Ordering::SeqCst);
            });
        }

        thread_pool.join();
        Ok(())
    }

    fn collect_results(rx: mpsc::Receiver<io::Result<Vec<SlowQueryRow>>>) -> io::Result<Vec<SlowQueryRow>> {
        let mut results = Vec::new();
        while let Ok(result) = rx.recv() {
            results.extend(result?);
        }
        Ok(results)
    }

    pub fn parse_data_for_slow_log(
        &self,
        file_list: Vec<String>,
        concurrency: usize,
        batch_size: usize,
    ) -> io::Result<Vec<SlowQueryRow>> {
        let mut results = Vec::new();
        let file_count = file_list.len();
        let (tx, rx) = mpsc::sync_channel(concurrency);

        thread::scope(|s| {
            for file_path in file_list.into_iter().rev() {
                let tx = tx.clone();
                s.spawn(move || {
                    let reader = match std::fs::File::open(&file_path) {
                        Ok(file) => std::io::BufReader::new(file),
                        Err(e) => {
                            tracing::warn!("Failed to open file {}: {}", file_path, e);
                            return;
                        }
                    };
                    
                    match self.parse_slow_log_concurrent(reader, concurrency, batch_size) {
                        Ok(rows) => tx.send(Ok(rows)).unwrap(),
                        Err(e) => {
                            tracing::error!("Failed to parse log file {}: {}", file_path, e);
                            tx.send(Err(e)).unwrap();
                        }
                    }
                });
            }
        });

        for _ in 0..file_count {
            if let Ok(result) = rx.recv() {
                results.extend(result?);
            }
        }
        
        Ok(results)
    }
}
