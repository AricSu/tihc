use std::fs::*;
use std::process::Command;

use crate::util::time::*;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct GrafanaImage {
    pub grafana_name: String,
    pub grafana_url: String,
    pub image_path: String,
}

impl GrafanaImage {
    pub fn new(
        grafana_name_param: String,
        grafana_url_param: String,
        image_path_param: String,
    ) -> Self {
        return GrafanaImage {
            grafana_name: grafana_name_param,
            grafana_url: grafana_url_param,
            image_path: image_path_param,
        };
    }
}

pub fn gen_all_image(
    path: String,
    login_name: String,
    login_passwd: String,
    ip: String,
    port: u64,
    start_time: u64,
    end_time: u64,
) -> Vec<GrafanaImage> {
    let url_overview_tidb_service_port_status = GrafanaImage::new("url_overview_tidb_service_port_status".to_string(), 
    format!("curl -o {}/{}.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\&to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,"url_overview_tidb_service_port_status".to_string(),login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    let url_overview_pd_storage_capacity = GrafanaImage::new("url_overview_pd_storage_capacity".to_string(), 
    format!("curl -o {}/{}.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/grafana-overview\\?orgId\\=1\\&refresh=30s\\&from\\={}\\&to\\={}\\&panelId\\=27\\&width\\=1000&height\\=500&tz\\=Asia%2FShanghai",path,"url_overview_pd_storage_capacity".to_string(),login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_pd_current_storage = GrafanaImage::new("url_overview_pd_current_storage".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_pd_region_healthy = GrafanaImage::new("url_overview_pd_region_healthy".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_tidb_sql_duration = GrafanaImage::new("url_overview_tidb_sql_duration".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_tidb_connection_count = GrafanaImage::new("url_overview_tidb_connection_count".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_tidb_transaction_ops = GrafanaImage::new("url_overview_tidb_transaction_ops".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_tidb_pd_tso_wait_duration = GrafanaImage::new("url_overview_tidb_pd_tso_wait_duration".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_tikv_leader = GrafanaImage::new("url_overview_tikv_leader".to_string(),
    // format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_tikv_region = GrafanaImage::new("url_overview_tikv_region".to_string(),
    // format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_systeminfo_cpu_usage = GrafanaImage::new("url_overview_systeminfo_cpu_usage".to_string(),
    // format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_systeminfo_memory_avaliable = GrafanaImage::new("url_overview_systeminfo_memory_avaliable".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_systeminfo_network_traffic = GrafanaImage::new("url_overview_systeminfo_network_traffic".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_overview_systeminfo_io_util = GrafanaImage::new("url_overview_systeminfo_io_util".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tidb_server_uptime = GrafanaImage::new("url_tidb_server_uptime".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tidb_executor_parse_duration = GrafanaImage::new("url_tidb_executor_parse_duration".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tidb_executor_compile_duration = GrafanaImage::new("url_tidb_executor_compile_duration".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_cluster_capacity_size = GrafanaImage::new("url_tikvdetail_cluster_capacity_size".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_threadcpu_raft_store_cpu = GrafanaImage::new("url_tikvdetail_threadcpu_raft_store_cpu".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_threadcpu_async_apply_cpu = GrafanaImage::new("url_tikvdetail_threadcpu_async_apply_cpu".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_threadcpu_scheduler_worker_cpu = GrafanaImage::new("url_tikvdetail_threadcpu_scheduler_worker_cpu".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_threadcpu_grpc_poll_cpu = GrafanaImage::new("url_tikvdetail_threadcpu_grpc_poll_cpu".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_threadcpu_unified_read_poll_cpu = GrafanaImage::new("url_tikvdetail_threadcpu_unified_read_poll_cpu".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_threadcpu_storage_read_poll_cpu = GrafanaImage::new("url_tikvdetail_threadcpu_storage_read_poll_cpu".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_threadcpu_coprocessor_cpu = GrafanaImage::new("url_tikvdetail_threadcpu_coprocessor_cpu".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_storage_async_snapshot_duration = GrafanaImage::new("url_tikvdetail_storage_async_snapshot_duration".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    //     let url_tikvdetail_storage_async_write_duration = GrafanaImage::new("url_tikvdetail_storage_async_write_duration".to_string(),
    //     format!("curl -o {}/02_SQL_Duration.png http://{}:{}@{}:{}/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\={}\\& \
    //     to\\={}\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai",path,login_name,login_passwd,ip,port,start_time,end_time), path.clone());

    return vec![
        url_overview_tidb_service_port_status,
        url_overview_pd_storage_capacity,
        // url_overview_pd_current_storage,
        // url_overview_pd_region_healthy,
        // url_overview_tidb_sql_duration,
        // url_overview_tidb_connection_count,
        // url_overview_tidb_transaction_ops,
        // url_overview_tidb_pd_tso_wait_duration,
        // url_overview_tikv_leader,
        // url_overview_tikv_region,
        // url_overview_systeminfo_cpu_usage,
        // url_overview_systeminfo_memory_avaliable,
        // url_overview_systeminfo_network_traffic,
        // url_overview_systeminfo_io_util,
        // url_tidb_server_uptime,
        // url_tidb_executor_parse_duration,
        // url_tidb_executor_compile_duration,
        // url_tikvdetail_cluster_capacity_size,
        // url_tikvdetail_threadcpu_raft_store_cpu,
        // url_tikvdetail_threadcpu_async_apply_cpu,
        // url_tikvdetail_threadcpu_scheduler_worker_cpu,
        // url_tikvdetail_threadcpu_grpc_poll_cpu,
        // url_tikvdetail_threadcpu_unified_read_poll_cpu,
        // url_tikvdetail_threadcpu_storage_read_poll_cpu,
        // url_tikvdetail_threadcpu_coprocessor_cpu,
        // url_tikvdetail_storage_async_snapshot_duration,
        // url_tikvdetail_storage_async_write_duration,
    ];
}

pub fn get_all_panel_image() {
    let start_time: DateTime<Utc> = Utc::now();
    let _hash_time = calculate_hash(&start_time);
    // let image_path = format!(
    //     "/tmp/ticheck_grafana_image_{}",
    //     hash_time.checked_div(321456).unwrap()
    // );
    let image_path = "/tmp/ticheck_image_dir".to_string();

    let _ = create_dir(&image_path);

    let all_images = gen_all_image(
        image_path.clone(),
        "admin".to_string(),
        "admin".to_string(),
        "localhost".to_string(),
        3000,
        1641203654867,
        1641203954867,
    );

    for i in all_images {
        let output = Command::new("sh")
            .arg("-c")
            .arg(i.grafana_url)
            .output()
            .expect("sh exec error!");
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    }
}
