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
    let image1 = DocType::Patagraph(gen_image(
        "url_overview_tidb_service_port_status".to_string(),
    ));
    let header_4_4_1 = DocType::Patagraph(gen_heading("4.4.1 PD概览", 20, 3));
    let image2 = DocType::Patagraph(gen_image("url_overview_pd_storage_capacity".to_string()));
    let header_4_4_2 = DocType::Patagraph(gen_heading("4.4.2 TiDB概览", 20, 3));
    let image3 = DocType::Patagraph(gen_image("url_overview_tidb_sql_duration".to_string()));
    let header_4_4_3 = DocType::Patagraph(gen_heading("4.4.3 TiKV概览", 20, 3));
    let header_4_4_4 = DocType::Patagraph(gen_heading("4.4.4 系统信息概览", 20, 3));
    let header_4_5 = DocType::Patagraph(gen_heading("4.5 数据库日志", 30, 2));
    return vec![
        header_4,
        image1,
        header_4_1,
        text1,
        text2,
        header_4_2,
        header_4_3,
        header_4_3_1,
        header_4_3_2,
        header_4_4,
        header_4_4_1,
        image2,
        header_4_4_2,
        image3,
        header_4_4_3,
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
