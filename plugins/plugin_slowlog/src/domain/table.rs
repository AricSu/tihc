use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlowQueryRow {
    pub time: String,
    pub txn_start_ts: u64,
    pub user: String,
    pub host: String,
    pub conn_id: u64,                       // bigint unsigned
    pub session_alias: String,              // varchar(64)
    pub exec_retry_count: u64,              // bigint unsigned
    pub exec_retry_time: f64,               // double
    pub query_time: f64,                    // double
    pub parse_time: f64,                    // double
    pub compile_time: f64,                  // double
    pub rewrite_time: f64,                  // double
    pub preproc_subqueries: u64,            // bigint unsigned
    pub preproc_subqueries_time: f64,       // double
    pub optimize_time: f64,                 // double
    pub wait_ts: f64,                       // double
    pub prewrite_time: f64,                 // double
    pub wait_prewrite_binlog_time: f64,     // double
    pub commit_time: f64,                   // double
    pub get_commit_ts_time: f64,            // double
    pub commit_backoff_time: f64,           // double
    pub backoff_types: String,              // varchar(64)
    pub resolve_lock_time: f64,             // double
    pub local_latch_wait_time: f64,         // double
    pub write_keys: i64,                    // bigint
    pub write_size: i64,                    // bigint
    pub prewrite_region: i64,               // bigint
    pub txn_retry: i64,                     // bigint
    pub cop_time: f64,                      // double
    pub process_time: f64,                  // double
    pub wait_time: f64,                     // double
    pub backoff_time: f64,                  // double
    pub lock_keys_time: f64,                // double
    pub request_count: u64,                 // bigint unsigned
    pub total_keys: u64,                    // bigint unsigned
    pub process_keys: u64,                  // bigint unsigned
    pub rocksdb_delete_skipped_count: u64,  // bigint unsigned
    pub rocksdb_key_skipped_count: u64,     // bigint unsigned
    pub rocksdb_block_cache_hit_count: u64, // bigint unsigned
    pub rocksdb_block_read_count: u64,      // bigint unsigned
    pub rocksdb_block_read_byte: u64,       // bigint unsigned
    pub db: String,                         // varchar(64)
    pub index_names: String,                // varchar(100)
    pub is_internal: bool,                  // tinyint(1)
    pub digest: String,                     // varchar(64)
    pub stats: String,                      // varchar(512)
    pub cop_proc_avg: f64,                  // double
    pub cop_proc_p90: f64,                  // double
    pub cop_proc_max: f64,                  // double
    pub cop_proc_addr: String,              // varchar(64)
    pub cop_wait_avg: f64,                  // double
    pub cop_wait_p90: f64,                  // double
    pub cop_wait_max: f64,                  // double
    pub cop_wait_addr: String,              // varchar(64)
    pub mem_max: i64,                       // bigint
    pub disk_max: i64,                      // bigint
    pub kv_total: f64,                      // double
    pub pd_total: f64,                      // double
    pub backoff_total: f64,                 // double
    pub write_sql_response_total: f64,      // double
    pub result_rows: i64,                   // bigint
    pub warnings: String,                   // longtext
    pub backoff_detail: String,             // varchar(4096)
    pub prepared: bool,                     // tinyint(1)
    pub succ: bool,                         // tinyint(1)
    pub is_explicit_txn: bool,              // tinyint(1)
    pub is_write_cache_table: bool,         // tinyint(1)
    pub plan_from_cache: bool,              // tinyint(1)
    pub plan_from_binding: bool,            // tinyint(1)
    pub has_more_results: bool,             // tinyint(1)
    pub resource_group: String,             // varchar(64)
    pub request_unit_read: f64,             // double
    pub request_unit_write: f64,            // double
    pub time_queued_by_rc: f64,             // double
    pub tidb_cpu_time: f64,                 // double
    pub tikv_cpu_time: f64,                 // double
    pub plan: String,                       // longtext
    pub plan_digest: String,                // varchar(128)
    pub binary_plan: String,                // longtext
    pub prev_stmt: String,                  // longtext
    pub query: String,                      // longtext
}
