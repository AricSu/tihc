extern crate docx_rs;
use crate::components::sysinfo::system::*;
use crate::util::table::*;
use docx_rs::*;

pub fn gen_chapter_system_3(cluster_nodes: &ClusterSysInfo) -> Vec<DocType> {
    let header_3 = DocType::Patagraph(gen_heading("三、系统配置", 40, 1));
    let header_3_1 = DocType::Patagraph(gen_heading("3.1 系统概况", 30, 2));
    let header_3_2 = DocType::Patagraph(gen_heading("3.2 操作系统", 30, 2));
    let header_3_2_1 = DocType::Patagraph(gen_heading("3.2.1 操作系统版本信息", 20, 3));
    let text1 = DocType::Patagraph(gen_text(
        "目的：防止操作系统版本不一致，或是用到了有 bug 的内核版本。",
        20,
        "black",
    ));
    // generate table rows
    let table_header_sys_version =
        gen_table_header(vec!["节点 IP", "操作系统版本", "操作系统内核版本"]);
    let mut tb_rows_below_sys_version = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(
            vec![&node.sys_host, &node.sys_version, &node.kernel_version],
            20,
            "black",
        );
        tb_rows_below_sys_version.append(&mut vec![row]);
    }

    let table_sys_version = gen_table(
        table_header_sys_version,
        &mut tb_rows_below_sys_version,
        vec![1500, 1500, 5000],
        TableLayoutType::Fixed,
        250,
    );
    let table_sys_version = DocType::Table(table_sys_version);
    let header_3_2_2 = DocType::Patagraph(gen_heading("3.2.2 内核参数配置信息", 20, 3));
    // generate table rows
    let table_header_sys_conf = gen_table_header(vec!["节点 IP", "sysctl.conf 配置信息"]);
    let mut tb_rows_below_sys_conf = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(vec![&node.sys_host, &node.sys_conf], 20, "black");
        tb_rows_below_sys_conf.append(&mut vec![row]);
    }

    let tb_rows_below_sys_conf = gen_table(
        table_header_sys_conf,
        &mut tb_rows_below_sys_conf,
        vec![1500, 6500],
        TableLayoutType::Fixed,
        250,
    );
    let table_sys_conf = DocType::Table(tb_rows_below_sys_conf);
    let header_3_2_3 = DocType::Patagraph(gen_heading("3.2.3 Ulimit 参数配置信息", 20, 3));
    // generate table rows
    let table_header_sys_limit = gen_table_header(vec!["节点 IP", "sysctl.conf 配置信息"]);
    let mut tb_rows_below_sys_limit = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(vec![&node.sys_host, &node.sys_limit], 20, "black");
        tb_rows_below_sys_limit.append(&mut vec![row]);
    }

    let tb_rows_below_sys_limit = gen_table(
        table_header_sys_limit,
        &mut tb_rows_below_sys_limit,
        vec![1500, 6500],
        TableLayoutType::Fixed,
        250,
    );
    let table_sys_limit = DocType::Table(tb_rows_below_sys_limit);
    let header_3_2_4 = DocType::Patagraph(gen_heading("3.2.4 Swap 状态信息", 20, 3));
    // generate table rows
    let table_header_sys_swap = gen_table_header(vec!["节点 IP", "Swap 状态"]);
    let mut tb_rows_below_sys_limit = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(vec![&node.sys_host, &node.swap_status], 20, "black");
        tb_rows_below_sys_limit.append(&mut vec![row]);
    }

    let tb_rows_below_sys_swap = gen_table(
        table_header_sys_swap,
        &mut tb_rows_below_sys_limit,
        vec![1500, 6500],
        TableLayoutType::Fixed,
        250,
    );
    let table_sys_swap = DocType::Table(tb_rows_below_sys_swap);
    let header_3_2_5 = DocType::Patagraph(gen_heading("3.2.5 磁盘调度策略信息", 20, 3));
    // generate table rows
    let table_header_sys_io_schedule = gen_table_header(vec!["节点 IP", "磁盘路径", "调度策略"]);
    let mut tb_rows_below_sys_io_schedule = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(
            vec![
                &node.sys_host,
                &"<由于实现问题，暂时需要手动查看>".to_string(),
                &"<由于实现问题，暂时需要手动查看>".to_string(),
            ],
            20,
            "black",
        );
        tb_rows_below_sys_io_schedule.append(&mut vec![row]);
    }

    let tb_rows_below_io_schedule = gen_table(
        table_header_sys_io_schedule,
        &mut tb_rows_below_sys_io_schedule,
        vec![1500, 2500, 4000],
        TableLayoutType::Fixed,
        250,
    );
    let table_sys_io_schedule = DocType::Table(tb_rows_below_io_schedule);
    let header_3_2_6 = DocType::Patagraph(gen_heading("3.2.6 透明大页配置信息", 20, 3));
    // generate table rows
    let table_header_sys_thp_status = gen_table_header(vec!["节点 IP", "透明大页状态"]);
    let mut tb_rows_below_sys_thp_status = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(vec![&node.sys_host, &node.thp_status], 20, "black");
        tb_rows_below_sys_thp_status.append(&mut vec![row]);
    }

    let tb_rows_below_thp_status = gen_table(
        table_header_sys_thp_status,
        &mut tb_rows_below_sys_thp_status,
        vec![1500, 6500],
        TableLayoutType::Fixed,
        250,
    );
    let table_sys_thp_status = DocType::Table(tb_rows_below_thp_status);
    let header_3_2_7 = DocType::Patagraph(gen_heading("3.2.7 NTP服务状态信息", 20, 3));
    // generate table rows
    let table_header_ntp_status = gen_table_header(vec!["节点 IP", "NTP 状态"]);
    let mut tb_rows_below_ntp_status = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(vec![&node.sys_host, &node.ntp_status], 20, "black");
        tb_rows_below_ntp_status.append(&mut vec![row]);
    }

    let tb_rows_below_ntp_status = gen_table(
        table_header_ntp_status,
        &mut tb_rows_below_ntp_status,
        vec![1500, 6500],
        TableLayoutType::Fixed,
        250,
    );
    let table_ntp_status = DocType::Table(tb_rows_below_ntp_status);
    let header_3_2_8 = DocType::Patagraph(gen_heading("3.2.8 磁盘挂载参数信息", 20, 3));
    // generate table rows
    let table_header_sys_disk_mount = gen_table_header(vec!["节点 IP", "磁盘路径", "调度策略"]);
    let mut tb_rows_below_sys_disk_mount = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(
            vec![
                &node.sys_host,
                &"<由于实现问题，暂时需要手动查看>".to_string(),
                &"<由于实现问题，暂时需要手动查看>".to_string(),
            ],
            20,
            "black",
        );
        tb_rows_below_sys_disk_mount.append(&mut vec![row]);
    }

    let tb_rows_below_disk_mount = gen_table(
        table_header_sys_disk_mount,
        &mut tb_rows_below_sys_disk_mount,
        vec![1500, 2500, 4000],
        TableLayoutType::Fixed,
        250,
    );
    let table_sys_disk_mount = DocType::Table(tb_rows_below_disk_mount);
    let header_3_2_9 = DocType::Patagraph(gen_heading("3.2.9 防火墙运行状态信息", 20, 3));
    // generate table rows
    let table_header_sys_firewall_status = gen_table_header(vec!["节点 IP", "防火墙状态"]);
    let mut tb_rows_below_sys_firewall_status = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(vec![&node.sys_host, &node.firewalld_status], 20, "black");
        tb_rows_below_sys_firewall_status.append(&mut vec![row]);
    }

    let tb_rows_below_firewall_status = gen_table(
        table_header_sys_firewall_status,
        &mut tb_rows_below_sys_firewall_status,
        vec![1500, 6500],
        TableLayoutType::Fixed,
        250,
    );
    let table_sys_firewall_status = DocType::Table(tb_rows_below_firewall_status);
    let header_3_2_10 = DocType::Patagraph(gen_heading("3.2.10 CPU 运行模式信息", 20, 3));
    // generate table rows
    let table_header_cpu_mode = gen_table_header(vec!["节点 IP", "CPU 模式"]);
    let mut tb_rows_below_cpu_mode = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(vec![&node.sys_host, &node.cpu_mode], 20, "black");
        tb_rows_below_cpu_mode.append(&mut vec![row]);
    }

    let tb_rows_below_cpu_mode = gen_table(
        table_header_cpu_mode,
        &mut tb_rows_below_cpu_mode,
        vec![1500, 6500],
        TableLayoutType::Fixed,
        250,
    );
    let table_cpu_mode = DocType::Table(tb_rows_below_cpu_mode);

    return vec![
        header_3,
        header_3_1,
        header_3_2,
        header_3_2_1,
        text1,
        table_sys_version,
        header_3_2_2,
        table_sys_conf,
        header_3_2_3,
        table_sys_limit,
        header_3_2_4,
        table_sys_swap,
        header_3_2_5,
        table_sys_io_schedule,
        header_3_2_6,
        table_sys_thp_status,
        header_3_2_7,
        table_ntp_status,
        header_3_2_8,
        table_sys_disk_mount,
        header_3_2_9,
        table_sys_firewall_status,
        header_3_2_10,
        table_cpu_mode,
    ];
}
