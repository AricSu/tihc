extern crate docx_rs;
use crate::components::sysinfo::system::*;
use crate::util::table::*;
use docx_rs::*;

pub fn gen_chapter_system(cluster_nodes: &ClusterSysInfo) -> Vec<DocType> {
    // generate table header for system version
    let table_header = gen_table_header(vec!["节点 IP", "操作系统版本", "操作系统内核版本"]);

    // generate table rows
    let mut tb_rows_below = vec![];
    for node in &cluster_nodes.all_nodes {
        let row = gen_table_row(vec![
            &node.sys_host,
            &node.sys_version,
            &node.kernel_version,
        ]);
        tb_rows_below.append(&mut vec![row]);
    }

    let table = gen_table(
        table_header,
        &mut tb_rows_below,
        vec![1500, 1500, 5000],
        TableLayoutType::Fixed,
        250,
    );

    let header1 = DocType::Patagraph(gen_heading("三、系统配置", 40, 1));
    let header2 = DocType::Patagraph(gen_heading("3.2 操作系统", 30, 2));
    let header3 = DocType::Patagraph(gen_heading("3.2.1 操作系统版本信息", 20, 3));
    let text1 = DocType::Patagraph(gen_text(
        "目的：防止操作系统版本不一致，或是用到了有 bug 的内核版本。",
    ));
    let table1 = DocType::Table(table);
    return vec![header1, header2, header3, text1, table1];
}
