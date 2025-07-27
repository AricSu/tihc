pub struct SlowLogFields;

impl SlowLogFields {
    // Basic components for constructing prefixes
    pub(crate) const ROW_PREFIX: &'static str = "# ";
    pub(crate) const SPACE_MARK: &'static str = ": ";
    pub(crate) const TIME: &'static str = "Time";

    // Constructed prefix (equivalent to SlowLogStartPrefixStr)
    pub const START_PREFIX: &'static str = "# Time: ";
    pub const SQL_SUFFIX: &'static str = ";";
    pub const TXN_START_TS: &'static str = "Txn_start_ts";
    pub const KEYSPACE_NAME: &'static str = "Keyspace_name";
    pub const KEYSPACE_ID: &'static str = "Keyspace_ID";
    pub const USER_AND_HOST: &'static str = "User@Host";
    pub const USER: &'static str = "User";
    pub const HOST: &'static str = "Host";
    pub const CONN_ID: &'static str = "Conn_ID";
    pub const SESSION_ALIAS: &'static str = "Session_alias";
    pub const QUERY_TIME: &'static str = "Query_time";
    pub const PARSE_TIME: &'static str = "Parse_time";
    pub const COMPILE_TIME: &'static str = "Compile_time";
    pub const REWRITE_TIME: &'static str = "Rewrite_time";
    pub const OPTIMIZE_TIME: &'static str = "Optimize_time";
    pub const WAIT_TS_TIME: &'static str = "Wait_TS";
    pub const PREPROC_SUBQUERIES: &'static str = "Preproc_subqueries";
    pub const PREPROC_SUBQUERIES_TIME: &'static str = "Preproc_subqueries_time";
    pub const DB: &'static str = "DB";
    pub const IS_INTERNAL: &'static str = "Is_internal";
    pub const DIGEST: &'static str = "Digest";
    pub const INDEX_NAMES: &'static str = "Index_names";
    pub const QUERY: &'static str = "Query";
    pub const STATS_INFO: &'static str = "Stats";
    pub const NUM_COP_TASKS: &'static str = "Num_cop_tasks";
    pub const COP_PROC_AVG: &'static str = "Cop_proc_avg";
    pub const COP_PROC_P90: &'static str = "Cop_proc_p90";
    pub const COP_PROC_MAX: &'static str = "Cop_proc_max";
    pub const COP_PROC_ADDR: &'static str = "Cop_proc_addr";
    pub const COP_WAIT_AVG: &'static str = "Cop_wait_avg";
    pub const COP_WAIT_P90: &'static str = "Cop_wait_p90";
    pub const COP_WAIT_MAX: &'static str = "Cop_wait_max";
    pub const COP_WAIT_ADDR: &'static str = "Cop_wait_addr";
    pub const COP_BACKOFF_PREFIX: &'static str = "Cop_backoff_";
    pub const PLAN: &'static str = "Plan";
    pub const PLAN_DIGEST: &'static str = "Plan_digest";
    pub const PREV_STMT: &'static str = "Prev_stmt";
    pub const KV_TOTAL: &'static str = "KV_total";
    pub const PD_TOTAL: &'static str = "PD_total";
    pub const BACKOFF_TOTAL: &'static str = "Backoff_total";
    pub const WRITE_KEYS: &'static str = "Write_keys";
    pub const WRITE_SIZE: &'static str = "Write_size";
    pub const PREWRITE_REGION: &'static str = "Prewrite_region";
    pub const TXN_RETRY: &'static str = "Txn_retry";
    pub const REQUEST_COUNT: &'static str = "Request_count";
    pub const TOTAL_KEYS: &'static str = "Total_keys";
    pub const PROCESS_KEYS: &'static str = "Process_keys";
    pub const ROCKSDB_DELETE_SKIPPED_COUNT: &'static str = "Rocksdb_delete_skipped_count";
    pub const ROCKSDB_KEY_SKIPPED_COUNT: &'static str = "Rocksdb_key_skipped_count";
    pub const ROCKSDB_BLOCK_CACHE_HIT_COUNT: &'static str = "Rocksdb_block_cache_hit_count";
    pub const ROCKSDB_BLOCK_READ_COUNT: &'static str = "Rocksdb_block_read_count";
    pub const ROCKSDB_BLOCK_READ_BYTE: &'static str = "Rocksdb_block_read_byte";
    pub const PREWRITE_TIME: &'static str = "Prewrite_time";
    pub const WAIT_PREWRITE_BINLOG_TIME: &'static str = "Wait_prewrite_binlog_time";
    pub const COMMIT_TIME: &'static str = "Commit_time";
    pub const GET_COMMIT_TS_TIME: &'static str = "Get_commit_ts_time";
    pub const COMMIT_BACKOFF_TIME: &'static str = "Commit_backoff_time";
    pub const RESOLVE_LOCK_TIME: &'static str = "Resolve_lock_time";
    pub const LOCAL_LATCH_WAIT_TIME: &'static str = "Local_latch_wait_time";
    pub const WRITE_SQL_RESPONSE_TOTAL: &'static str = "Write_sql_response_total";
    pub const BACKOFF_TYPES: &'static str = "Backoff_types";
    pub const COP_TIME: &'static str = "Cop_time";
    pub const PROCESS_TIME: &'static str = "Process_time";
    pub const WAIT_TIME: &'static str = "Wait_time";
    pub const BACKOFF_TIME: &'static str = "Backoff_time";
    pub const LOCK_KEYS_TIME: &'static str = "Lock_keys_time";
    pub const SUCC: &'static str = "Succ";
    pub const PLAN_FROM_BINDING: &'static str = "Plan_from_binding";
    pub const HAS_MORE_RESULTS: &'static str = "Has_more_results";
    pub const MEM_MAX: &'static str = "Mem_max";
    pub const DISK_MAX: &'static str = "Disk_max";
    pub const PREPARED: &'static str = "Prepared";
    pub const PLAN_FROM_CACHE: &'static str = "Plan_from_cache";
    pub const BINARY_PLAN: &'static str = "Binary_plan";
    pub const PLAN_PREFIX: &'static str = "DECODE_PLAN('";
    pub const BINARY_PLAN_PREFIX: &'static str = "DECODE_BINARY_PLAN('";
    pub const PLAN_SUFFIX: &'static str = "')";
    pub const PREV_STMT_PREFIX: &'static str = "Prev_stmt: ";
    pub const UNPACKED_BYTES_SENT_TIKV_TOTAL: &'static str = "Unpacked_bytes_sent_tikv_total";
    pub const UNPACKED_BYTES_RECEIVED_TIKV_TOTAL: &'static str =
        "Unpacked_bytes_received_tikv_total";
    pub const UNPACKED_BYTES_SENT_TIKV_CROSS_ZONE: &'static str =
        "Unpacked_bytes_sent_tikv_cross_zone";
    pub const UNPACKED_BYTES_RECEIVED_TIKV_CROSS_ZONE: &'static str =
        "Unpacked_bytes_received_tikv_cross_zone";
    pub const UNPACKED_BYTES_SENT_TIFLASH_TOTAL: &'static str = "Unpacked_bytes_sent_tiflash_total";
    pub const UNPACKED_BYTES_RECEIVED_TIFLASH_TOTAL: &'static str =
        "Unpacked_bytes_received_tiflash_total";
    pub const UNPACKED_BYTES_SENT_TIFLASH_CROSS_ZONE: &'static str =
        "Unpacked_bytes_sent_tiflash_cross_zone";
    pub const UNPACKED_BYTES_RECEIVED_TIFLASH_CROSS_ZONE: &'static str =
        "Unpacked_bytes_received_tiflash_cross_zone";
    pub const EXEC_RETRY_COUNT: &'static str = "Exec_retry_count";
    pub const EXEC_RETRY_TIME: &'static str = "Exec_retry_time";
    pub const BACKOFF_DETAIL: &'static str = "Backoff_Detail";
    pub const RESULT_ROWS: &'static str = "Result_rows";
    pub const WARNINGS: &'static str = "Warnings";
    pub const IS_EXPLICIT_TXN: &'static str = "IsExplicitTxn";
    pub const IS_WRITE_CACHE_TABLE: &'static str = "IsWriteCacheTable";
    pub const IS_SYNC_STATS_FAILED: &'static str = "IsSyncStatsFailed";
    pub const RESOURCE_GROUP: &'static str = "Resource_group";
    pub const REQUEST_UNIT_READ: &'static str = "Request_unit_read";
    pub const REQUEST_UNIT_WRITE: &'static str = "Request_unit_write";
    pub const TIME_QUEUED_BY_RC: &'static str = "Time_queued_by_rc";
    pub const TIDB_CPU_USAGE_DURATION: &'static str = "Tidb_cpu_time";
    pub const TIKV_CPU_USAGE_DURATION: &'static str = "Tikv_cpu_time";
}
