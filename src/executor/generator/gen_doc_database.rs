extern crate docx_rs;
use crate::util::table::*;

pub fn gen_chapter_system_4() -> Vec<DocType> {
    let header_4 = DocType::Patagraph(gen_heading("四、数据库集群配置", 40, 1));
    let header_4_1 = DocType::Patagraph(gen_heading("4.1 TiDB系统service清单", 30, 2));
    let text1 = DocType::Patagraph(gen_text(
        "    本次检查数据库为 <yyyy> 生产库系统。",
        20,
        "black",
    ));
    let text2 = DocType::Patagraph(gen_text("    本报告提供的检查和建议不涉及具体的数据库安全分析和应用程序细节。本次数据库涉及了 1 套 <N> 节点TiDB数据库的检查，在这次检查中对主机和数据库配置和数据库性能进行了总体分析，不针对具体某个应用性能。",20,"black"));
    let header_4_2 = DocType::Patagraph(gen_heading("4.2 组件清单配置", 30, 2));
    let header_4_3 = DocType::Patagraph(gen_heading("4.3 数据库配置", 30, 2));
    let header_4_3_1 = DocType::Patagraph(gen_heading("4.3.1 软件版本", 20, 3));
    let header_4_3_2 = DocType::Patagraph(gen_heading("4.3.2 数据库参数", 20, 3));
    let header_4_4 = DocType::Patagraph(gen_heading("4.4 集群概览", 30, 2));
    let image_tidb_service_port_status = DocType::Patagraph(gen_image(
        "url_overview_tidb_service_port_status".to_string(),
    ));
    let image_systeminfo_cpu_usage = DocType::Patagraph(gen_image("url_overview_systeminfo_cpu_usage".to_string()));
    let image_systeminfo_memory_avaliable = DocType::Patagraph(gen_image("url_overview_systeminfo_memory_avaliable".to_string()));
    let image_systeminfo_network_traffic = DocType::Patagraph(gen_image("url_overview_systeminfo_network_traffic".to_string()));
    let image_overview_systeminfo_io_util = DocType::Patagraph(gen_image("url_overview_systeminfo_io_util".to_string()));
    let header_4_4_1 = DocType::Patagraph(gen_heading("4.4.1 PD概览", 20, 3));
    let image_pd_storage_capacity = DocType::Patagraph(gen_image("url_overview_pd_storage_capacity".to_string()));
    let image_pd_pd_region_healthy = DocType::Patagraph(gen_image("url_overview_pd_region_healthy".to_string()));
    let header_4_4_2 = DocType::Patagraph(gen_heading("4.4.2 TiDB概览", 20, 3));
    let image_tidb_server_uptime = DocType::Patagraph(gen_image("url_tidb_server_uptime".to_string()));
    let image_tidb_sql_duration = DocType::Patagraph(gen_image("url_overview_tidb_sql_duration".to_string()));
    let image_tidb_transaction_ops = DocType::Patagraph(gen_image("url_overview_tidb_transaction_ops".to_string()));
    let image_tidb_pd_tso_wait_duration = DocType::Patagraph(gen_image("url_overview_tidb_pd_tso_wait_duration".to_string()));
    let image_tidb_executor_parse_duration = DocType::Patagraph(gen_image("url_tidb_executor_parse_duration".to_string()));
    let image_tidb_executor_compile_duration = DocType::Patagraph(gen_image("url_tidb_executor_compile_duration".to_string()));
    let header_4_4_3 = DocType::Patagraph(gen_heading("4.4.3 TiKV概览", 20, 3));
    let image_tikv_leader = DocType::Patagraph(gen_image("url_overview_tikv_leader".to_string()));
    let image_tikv_region = DocType::Patagraph(gen_image("url_overview_tikv_region".to_string()));
    let image_tikv_cluster_capacity_size = DocType::Patagraph(gen_image("url_tikvdetail_cluster_capacity_size".to_string()));
    let image_tikv_cluster_avaliable_size = DocType::Patagraph(gen_image("url_tikvdetail_cluster_avaliable_size".to_string()));
    let image_tikv_threadcpu_raft_store_cpu = DocType::Patagraph(gen_image("url_tikvdetail_threadcpu_raft_store_cpu".to_string()));
    let image_tikv_threadcpu_async_apply_cpu = DocType::Patagraph(gen_image("url_tikvdetail_threadcpu_async_apply_cpu".to_string()));
    let image_tikv_threadcpu_scheduler_worker_cpu = DocType::Patagraph(gen_image("url_tikvdetail_threadcpu_scheduler_worker_cpu".to_string()));
    let image_tikv_threadcpu_grpc_poll_cpu = DocType::Patagraph(gen_image("url_tikvdetail_threadcpu_grpc_poll_cpu".to_string()));
    let image_tikv_threadcpu_unified_read_poll_cpu = DocType::Patagraph(gen_image("url_tikvdetail_threadcpu_unified_read_poll_cpu".to_string()));
    let image_tikv_threadcpu_storage_read_poll_cpu = DocType::Patagraph(gen_image("url_tikvdetail_threadcpu_storage_read_poll_cpu".to_string()));
    let image_tikv_threadcpu_coprocessor_cpu = DocType::Patagraph(gen_image("url_tikvdetail_threadcpu_coprocessor_cpu".to_string()));
    let image_tikv_storage_async_snapshot_duration = DocType::Patagraph(gen_image("url_tikvdetail_storage_async_snapshot_duration".to_string()));
    let image_tikv_storage_async_write_duration = DocType::Patagraph(gen_image("url_tikvdetail_storage_async_write_duration".to_string()));
    let header_4_4_4 = DocType::Patagraph(gen_heading("4.4.4 系统信息概览", 20, 3));
    let header_4_5 = DocType::Patagraph(gen_heading("4.5 数据库日志", 30, 2));
    return vec![
        header_4,
        image_tidb_service_port_status,
        image_systeminfo_cpu_usage,
        image_systeminfo_memory_avaliable,
        image_systeminfo_network_traffic,
        image_overview_systeminfo_io_util,
        header_4_1,
        text1,
        text2,
        header_4_2,
        header_4_3,
        header_4_3_1,
        header_4_3_2,
        header_4_4,
        header_4_4_1,
        image_pd_storage_capacity,
        image_pd_pd_region_healthy,
        header_4_4_2,
        image_tidb_server_uptime,
        image_tidb_sql_duration,
        image_tidb_transaction_ops,
        image_tidb_pd_tso_wait_duration,
        image_tidb_executor_parse_duration,
        image_tidb_executor_compile_duration,
        header_4_4_3,
        image_tikv_leader,
        image_tikv_region,
        image_tikv_cluster_capacity_size,
        image_tikv_cluster_avaliable_size,
        image_tikv_threadcpu_raft_store_cpu,
        image_tikv_threadcpu_async_apply_cpu,
        image_tikv_threadcpu_scheduler_worker_cpu,
        image_tikv_threadcpu_grpc_poll_cpu,
        image_tikv_threadcpu_unified_read_poll_cpu,
        image_tikv_threadcpu_storage_read_poll_cpu,
        image_tikv_threadcpu_coprocessor_cpu,
        image_tikv_storage_async_snapshot_duration,
        image_tikv_storage_async_write_duration,
        header_4_4_4,
        header_4_5,
    ];
}

pub fn gen_chapter_system_5() -> Vec<DocType> {
    let header_5 = DocType::Patagraph(gen_heading("五、数据库性能", 40, 1));
    let header_5_1 = DocType::Patagraph(gen_heading("5.1 SQL性能概况", 30, 2));
    let header_5_2 = DocType::Patagraph(gen_heading("5.2 慢 SQL 1 根因分析", 30, 2));
    let header_5_3 = DocType::Patagraph(gen_heading("5.3 慢 SQL 2 根因分析", 30, 2));

    // .add_paragraph(gen_heading("六、备份与恢复", 40, 1))
    // .add_paragraph(gen_heading("七、容灾与高可用评估", 40, 1));
    return vec![header_5, header_5_1, header_5_2, header_5_3];
}
