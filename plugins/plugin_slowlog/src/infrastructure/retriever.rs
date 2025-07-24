use std::io;
use crate::domain::{fields::SlowLogFields, table::SlowQueryRow};

pub fn split_by_colon(line: &str) -> (Vec<String>, Vec<String>) {
    tracing::debug!("Starting to parse line: {}", line);

    fn is_letter_or_numeric(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }

    fn find_matched_right_bracket(line: &str, left_bracket_idx: usize) -> Option<usize> {
        let chars: Vec<char> = line.chars().collect();
        if left_bracket_idx >= chars.len() {
            return None;
        }

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
                    if current + 1 < chars.len() && chars[current + 1] != ' ' {
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
    let mut current = 0;
    let mut parse_key = true;
    let mut err_msg = None;

    let chars: Vec<char> = line.chars().collect();
    let line_length = chars.len();

    while current < line_length {
        if parse_key {
            // Find key start
            while current < line_length && !is_letter_or_numeric(chars[current]) {
                current += 1;
            }
            let start = current;
            if current >= line_length {
                break;
            }

            // Find colon position
            while current < line_length && chars[current] != ':' {
                current += 1;
            }

            if current < line_length {
                // Convert char indices to byte indices for string slicing
                let start_byte = chars[..start].iter().map(|c| c.len_utf8()).sum();
                let end_byte = chars[..current].iter().map(|c| c.len_utf8()).sum();

                fields.push(line[start_byte..end_byte].to_string());

                // Skip colon and space
                current += 2; // bypass ": "
                if current >= line_length {
                    // last empty value
                    values.push("".to_string());
                }
            }
            parse_key = false;
        } else {
            let start = current;
            if current < line_length && (chars[current] == '{' || chars[current] == '[') {
                // Convert char index to byte index
                let start_byte = chars[..start].iter().map(|c| c.len_utf8()).sum();

                if let Some(r_brace_idx) = find_matched_right_bracket(line, start) {
                    // Convert char index to byte index
                    let end_byte = chars[..=r_brace_idx].iter().map(|c| c.len_utf8()).sum();

                    values.push(line[start_byte..end_byte].to_string());
                    current = r_brace_idx + 1;
                } else {
                    err_msg = Some("Braces matched error");
                    break;
                }
            } else {
                let mut value_end = current;
                while value_end < line_length && chars[value_end] != ' ' {
                    value_end += 1;
                }

                // Meet empty value cases: "Key: Key:"
                if value_end > 0 && value_end < line_length && chars[value_end - 1] == ':' {
                    values.push("".to_string());
                    current = start;
                    parse_key = true;
                    continue;
                }

                // Convert char indices to byte indices
                let start_byte = chars[..start].iter().map(|c| c.len_utf8()).sum();
                let end_byte = chars[..value_end].iter().map(|c| c.len_utf8()).sum();

                values.push(line[start_byte..end_byte].to_string());
                current = value_end;
            }
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

        // Float types
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

        // Boolean types
        SlowLogFields::IS_INTERNAL => query.is_internal = value == "true",
        SlowLogFields::PREPARED => query.prepared = value == "true",
        SlowLogFields::SUCC => query.succ = value == "true",
        SlowLogFields::IS_EXPLICIT_TXN => query.is_explicit_txn = value == "true",
        SlowLogFields::IS_WRITE_CACHE_TABLE => query.is_write_cache_table = value == "true",
        SlowLogFields::PLAN_FROM_CACHE => query.plan_from_cache = value == "true",
        SlowLogFields::PLAN_FROM_BINDING => query.plan_from_binding = value == "true",
        SlowLogFields::HAS_MORE_RESULTS => query.has_more_results = value == "true",

        // String types
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


// Enable debug log in test
#[test]
fn test_parse_commit_slow_log() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();

    // ... existing test code ...
}
