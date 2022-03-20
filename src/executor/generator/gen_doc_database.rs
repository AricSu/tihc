extern crate docx_rs;
use crate::util::table::*;

pub fn gen_chapter_system_4() -> anyhow::Result<Vec<DocType>> {
    let header_4 = DocType::Patagraph(gen_heading("四、数据库集群配置", 40, 1));
    let header_4_1 = DocType::Patagraph(gen_heading("4.1 TiDB系统service清单", 30, 2));
    let image_port_aim = DocType::Patagraph(gen_text(
        "    目的：各服务在线节点数量,防止出现异常组件或异常节点。",
        20,
        "black",
    ));
    let image_port_location = DocType::Patagraph(gen_text(
        "    位置：tidb-master-overview -> Services Port Status -> Services Up",
        20,
        "black",
    ));
    let image_tidb_service_port_status = DocType::Patagraph(gen_image(
        "url_overview_tidb_service_port_status".to_string(),
    )?);
    let header_4_2 = DocType::Patagraph(gen_heading("4.2 组件清单配置", 30, 2));
    let text_4_2_1 = DocType::Patagraph(gen_text("    1. TiDB : 与应用进行交互负责 SQL 逻辑，通过 PD 寻址到实际数据的 TiKV 位置，进行 SQL 操作。",20,"black"));
    let text_4_2_2 = DocType::Patagraph(gen_text("    2. TiKV : 负责数据存储，是一个提供完整ACID事务的分布式 Key-Value 存储引擎。",20,"black"));
    let text_4_2_3 = DocType::Patagraph(gen_text("    3. PD : 负责管理调度，如数据和 TiKV 位置的路由信息维护、TiKV 数据均衡，TiDB通过pd获取tso。",20,"black"));
    let text_4_2_4 = DocType::Patagraph(gen_text("    4. TiFlash : TiKV 的列存扩展，拥有异步复制、一致性、智能选择、计算加速等几个核心特性。",20,"black"));
    let header_4_3 = DocType::Patagraph(gen_heading("4.3 数据库配置", 30, 2));
    let header_4_3_1 = DocType::Patagraph(gen_heading("4.3.1 软件版本", 20, 3));
    let header_4_3_2 = DocType::Patagraph(gen_heading("4.3.2 数据库参数", 20, 3));
    let header_4_4 = DocType::Patagraph(gen_heading("4.4 集群概览", 30, 2));

    let image_cpu_aim = DocType::Patagraph(gen_text(
        "    概览各组件资源消耗，防止网卡打满、IO打满、CPU打满、内存耗尽等问题发生。",
        20,
        "black",
    ));
    let image_cpu_location = DocType::Patagraph(gen_text(
        "    位置：tidb-master-Overview -> System Info -> CPU Usage",
        20,
        "black",
    ));
    let image_mem_location = DocType::Patagraph(gen_text(
        "          tidb-master-Overview -> System Info -> Memory Available",
        20,
        "black",
    ));
    let image_net_location = DocType::Patagraph(gen_text(
        "          tidb-master-Overview -> System Info -> Network Traffic ",
        20,
        "black",
    ));
    let image_io_location = DocType::Patagraph(gen_text(
        "          tidb-master-Overview -> System Info -> IO Util",
        20,
        "black",
    ));
    let image_systeminfo_cpu_usage =
        DocType::Patagraph(gen_image("url_overview_systeminfo_cpu_usage".to_string())?);
    let image_systeminfo_memory_avaliable = DocType::Patagraph(gen_image(
        "url_overview_systeminfo_memory_avaliable".to_string(),
    )?);
    let image_systeminfo_network_traffic = DocType::Patagraph(gen_image(
        "url_overview_systeminfo_network_traffic".to_string(),
    )?);
    let image_overview_systeminfo_io_util =
        DocType::Patagraph(gen_image("url_overview_systeminfo_io_util".to_string())?);
    let header_4_4_1 = DocType::Patagraph(gen_heading("4.4.1 PD概览", 20, 3));
    let image_pd_storage_capacity =
        DocType::Patagraph(gen_image("url_overview_pd_storage_capacity".to_string())?);

    let text_4_4_1_1 = DocType::Patagraph(gen_text("    1. Grafana PD storage 监控面板概览",20,"black"));
    let text_4_4_1_2 = DocType::Patagraph(gen_text("      目的：防止存储空间不足触发 PD 异常调度或组件宕机问题。",20,"black"));
    let text_4_4_1_3 = DocType::Patagraph(gen_text("      位置：tidb-master-overview -> PD -> Storage capacity",20,"black"));
    let text_4_4_1_4 = DocType::Patagraph(gen_text("            tidb-master-overview -> PD -> Current storage size",20,"black"));
    let image_pd_current_storage_size = DocType::Patagraph(gen_image("url_overview_pd_current_storage".to_string())?);
    let text_4_4_1_5 = DocType::Patagraph(gen_text("    2. Grafana PD Region 面板概览",20,"black"));
    let text_4_4_1_6 = DocType::Patagraph(gen_text("      目的：防止过多空 Region 导致 PD 异常调度问题。",20,"black"));
    let text_4_4_1_7 = DocType::Patagraph(gen_text("      位置：tidb-master-overview -> PD -> Region healthy",20,"black"));
    let image_pd_pd_region_healthy =
        DocType::Patagraph(gen_image("url_overview_pd_region_healthy".to_string())?);
    let header_4_4_2 = DocType::Patagraph(gen_heading("4.4.2 TiDB概览", 20, 3));
    let text_4_4_2_1 = DocType::Patagraph(gen_text("    1. Grafana TiDB 监控面板概览",20,"black"));
    let text_4_4_2_2 = DocType::Patagraph(gen_text("      目的：观察 statement OPS 分析业务是否存在 QPS 暴涨， SQL Duration 99 线是否合理。",20,"black"));
    let text_4_4_2_3 = DocType::Patagraph(gen_text("      位置：tidb-master-Overview -> TiDB -> Statement OPS",20,"black"));
    let text_4_4_2_4 = DocType::Patagraph(gen_text("            tidb-master-Overview -> TiDB -> Duration",20,"black"));

    let image_tidb_server_uptime =
        DocType::Patagraph(gen_image("url_tidb_server_uptime".to_string())?);
    let image_tidb_sql_duration =
        DocType::Patagraph(gen_image("url_overview_tidb_sql_duration".to_string())?);
    let image_tidb_transaction_ops =
        DocType::Patagraph(gen_image("url_overview_tidb_transaction_ops".to_string())?);
    let image_tidb_pd_tso_wait_duration = DocType::Patagraph(gen_image(
        "url_overview_tidb_pd_tso_wait_duration".to_string(),
    )?);
    let image_tidb_executor_parse_duration =
        DocType::Patagraph(gen_image("url_tidb_executor_parse_duration".to_string())?);
    let image_tidb_executor_compile_duration =
        DocType::Patagraph(gen_image("url_tidb_executor_compile_duration".to_string())?);
    let header_4_4_3 = DocType::Patagraph(gen_heading("4.4.3 TiKV概览", 20, 3));
    let image_tikv_leader = DocType::Patagraph(gen_image("url_overview_tikv_leader".to_string())?);
    let image_tikv_region = DocType::Patagraph(gen_image("url_overview_tikv_region".to_string())?);
    let image_tikv_cluster_capacity_size = DocType::Patagraph(gen_image(
        "url_tikvdetail_cluster_capacity_size".to_string(),
    )?);
    let image_tikv_cluster_avaliable_size = DocType::Patagraph(gen_image(
        "url_tikvdetail_cluster_avaliable_size".to_string(),
    )?);
    let image_tikv_threadcpu_raft_store_cpu = DocType::Patagraph(gen_image(
        "url_tikvdetail_threadcpu_raft_store_cpu".to_string(),
    )?);
    let image_tikv_threadcpu_async_apply_cpu = DocType::Patagraph(gen_image(
        "url_tikvdetail_threadcpu_async_apply_cpu".to_string(),
    )?);
    let image_tikv_threadcpu_scheduler_worker_cpu = DocType::Patagraph(gen_image(
        "url_tikvdetail_threadcpu_scheduler_worker_cpu".to_string(),
    )?);
    let image_tikv_threadcpu_grpc_poll_cpu = DocType::Patagraph(gen_image(
        "url_tikvdetail_threadcpu_grpc_poll_cpu".to_string(),
    )?);
    let image_tikv_threadcpu_unified_read_poll_cpu = DocType::Patagraph(gen_image(
        "url_tikvdetail_threadcpu_unified_read_poll_cpu".to_string(),
    )?);
    let image_tikv_threadcpu_storage_read_poll_cpu = DocType::Patagraph(gen_image(
        "url_tikvdetail_threadcpu_storage_read_poll_cpu".to_string(),
    )?);
    let image_tikv_threadcpu_coprocessor_cpu = DocType::Patagraph(gen_image(
        "url_tikvdetail_threadcpu_coprocessor_cpu".to_string(),
    )?);
    let image_tikv_storage_async_snapshot_duration = DocType::Patagraph(gen_image(
        "url_tikvdetail_storage_async_snapshot_duration".to_string(),
    )?);
    let image_tikv_storage_async_write_duration = DocType::Patagraph(gen_image(
        "url_tikvdetail_storage_async_write_duration".to_string(),
    )?);
    let header_4_4_4 = DocType::Patagraph(gen_heading("4.4.4 系统信息概览", 20, 3));
    let header_4_5 = DocType::Patagraph(gen_heading("4.5 数据库日志", 30, 2));
    Ok(vec![
        header_4,
        header_4_1,
        image_port_aim,
        image_port_location,
        image_tidb_service_port_status,
        header_4_2,
        text_4_2_1,
        text_4_2_2,
        text_4_2_3,
        text_4_2_4,
        header_4_3,
        header_4_3_1,
        header_4_3_2,
        header_4_4,
        header_4_4_1,
        text_4_4_1_1,
        text_4_4_1_2,
        text_4_4_1_3,
        text_4_4_1_4,
        image_pd_storage_capacity,
        image_pd_current_storage_size,
        text_4_4_1_5,
        text_4_4_1_6,
        text_4_4_1_7,
        image_pd_pd_region_healthy,
        header_4_4_2,
        text_4_4_2_1,
        text_4_4_2_2,
        text_4_4_2_3,
        text_4_4_2_4,
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
        image_cpu_aim,
        image_cpu_location,
        image_mem_location,
        image_net_location,
        image_io_location,
        image_systeminfo_cpu_usage,
        image_systeminfo_memory_avaliable,
        image_systeminfo_network_traffic,
        image_overview_systeminfo_io_util,
        header_4_5,
    ])
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
