extern crate docx_rs;
use crate::util::table::*;
use docx_rs::*;

pub fn gen_chapter_system_1() -> Vec<DocType> {
    let header_1 = DocType::Patagraph(gen_heading("一、检查介绍", 40, 1));
    let header_1_1 = DocType::Patagraph(gen_heading("1.1 检查系统", 30, 2));
    let text_1 = DocType::Patagraph(gen_text(
        "   本次检查数据库为 <yyyy> 生产库系统。",
        20,
        "red",
    ));
    let text_2 = DocType::Patagraph(gen_text("   本报告提供的检查和建议不涉及具体的数据库安全分析和应用程序细节。在这次检查中对主机和数据库配置和数据库性能进行了总体分析，不针对具体某个应用性能。",20,"black"));
    let header_1_2 = DocType::Patagraph(gen_heading("1.2 检查方法", 30, 2));
    let text3 = DocType::Patagraph(gen_text("    本次数据库性能检查的工具是：", 20, "black"));
    let text3_1 = DocType::Patagraph(gen_text(
        "        1.TiDB Dashboard、Prometheus、Grafana 进行系统信息收集;",
        20,
        "black",
    ));
    let text3_2 = DocType::Patagraph(gen_text(
        "        2.操作系统工具和命令检查操作系统;",
        20,
        "black",
    ));
    let text3_3 = DocType::Patagraph(gen_text("        3.SQL 命令检查数据库配置;", 20, "black"));
    let text3_4 = DocType::Patagraph(gen_text(
        "        4.TiDB Dashboard、Prometheus、Grafana 进行数据库性能资料的收集;",
        20,
        "black",
    ));
    let text4 = DocType::Patagraph(gen_text("    上述输出结果为建议提供依据。", 20, "black"));
    let header_1_3 = DocType::Patagraph(gen_heading("1.3 检查范围", 30, 2));
    let text5 = DocType::Patagraph(gen_text("    本次 TiDB 数据库的检查对主机和数据库配置和数据库性能进行了总体分析，不针对具体某个应用性能。",20,"black"));
    let text6 = DocType::Patagraph(gen_text(
        "    本报告提供的检查和建议不涉及具体的数据库安全分析和应用程序细节。",
        20,
        "black",
    ));
    return vec![
        header_1, header_1_1, text_1, text_2, header_1_2, text3, text3_1, text3_2, text3_3,
        text3_4, text4, header_1_3, text5, text6,
    ];
}

pub fn gen_chapter_system_2() -> Vec<DocType> {
    let header_2 = DocType::Patagraph(gen_heading("二、检查总结", 40, 1));
    let header_2_1 = DocType::Patagraph(gen_heading("2.1 操作系统配置建议", 30, 2));
    let text1 = DocType::Patagraph(gen_text(
        "    经查操作系统参数配置建议如下，详情参考 3.1 ～3.4：",
        20,
        "black",
    ));
    let text2 = DocType::Patagraph(gen_text(
        "    <对操作系统各项参数的建议,如无建议，请填写 无>",
        20,
        "Red",
    ));
    let header_2_2 = DocType::Patagraph(gen_heading("2.2 数据库版本建议", 30, 2));
    let text3 = DocType::Patagraph(gen_text(
        "    经查数据库版本建议如下，详情参考 4.3.1 ：",
        20,
        "black",
    ));
    let text4 = DocType::Patagraph(gen_text(
        "    <对所有组件版本给出建议，特别是生命周期快结束的版本建议升级,如无建议，请填写 无>",
        20,
        "Red",
    ));
    let header_2_3 = DocType::Patagraph(gen_heading("2.3 数据库参数建议", 30, 2));
    let text5 = DocType::Patagraph(gen_text(
        "    经查数据库参数建议如下，详情参考 4.3.2 ：",
        20,
        "black",
    ));
    let text6 = DocType::Patagraph(gen_text(
        "    <评审数据库参数，给出参数优化建议,如无建议，请填写 无>",
        20,
        "Red",
    ));
    let header_2_4 = DocType::Patagraph(gen_heading("2.4 数据库组件建议", 30, 2));
    let text7 = DocType::Patagraph(gen_text(
        "    经查数据库参数建议如下，详情参考4.4.1 ～4.4.4 ：",
        20,
        "black",
    ));
    let text8 = DocType::Patagraph(gen_text(
        "    <针对 PD、TiKV、TiDB 对各项组件参数的建议，包括总体存储扩容建议,如无建议，请填写 无>",
        20,
        "Red",
    ));
    let header_2_5 = DocType::Patagraph(gen_heading("2.5 数据库日志建议", 30, 2));
    let text9 = DocType::Patagraph(gen_text("    <贴出异常的告警，分析告警日志，给出建议。 如：数据库出现大量不影响应用的 Dispatch Faild SQL 建议应用及时处理掉>",20,"Red"));
    let header_2_6 = DocType::Patagraph(gen_heading("2.6 数据库管理建议", 30, 2));
    let table_header = gen_table_header(vec!["序 号", "事 件", "问 题 分 析", "建 议 及 目 的"]);
    let table_row_1 = gen_table_row(
        vec![
            &mut "1".to_string(),
            &mut "配置".to_string(),
            &mut "1. TiHC 部署路径只有一个".to_string(),
            &mut "建议增加控制文件分别放置于不同的存储路径之下，进行冗余".to_string(),
        ],
        20,
        "red",
    );
    let table_row_2 = gen_table_row(
        vec![
            &mut "2".to_string(),
            &mut "权限".to_string(),
            &mut "1. 有不少用户具有DBA权限".to_string(),
            &mut "建议对具有DBA权限的数据库用户进行定期审核，确保及时收回不必要的DBA权限"
                .to_string(),
        ],
        20,
        "red",
    );
    let table_row_3 = gen_table_row(
        vec![
            &mut "3".to_string(),
            &mut "资源".to_string(),
            &mut "1. 单台 TiDB 实例 Connection Count 历史值已达到最大值 1100".to_string(),
            &mut "建议增加 TiDB 实例扩瞳，避免出现客户端连接链接过多，处理阻塞情况发生".to_string(),
        ],
        20,
        "red",
    );
    let mut tb_rows_below = vec![];
    tb_rows_below.append(&mut vec![table_row_1]);
    tb_rows_below.append(&mut vec![table_row_2]);
    tb_rows_below.append(&mut vec![table_row_3]);

    // generate table rows

    let table = gen_table(
        table_header,
        &mut tb_rows_below,
        vec![1000, 1000, 2000, 3000],
        TableLayoutType::Fixed,
        500,
    );
    let table1 = DocType::Table(table);
    let header_2_7 = DocType::Patagraph(gen_heading("2.7 数据库备份建议", 30, 2));
    let text10 = DocType::Patagraph(gen_text("    <建议定期做恢复测试，并根据不同的数据库失败情况制定相应的恢复策略。如：数据库全库恢复、数据表恢复>",20,"Red"));
    return vec![
        header_2, header_2_1, text1, text2, header_2_2, text3, text4, header_2_3, text5, text6,
        header_2_4, text7, text8, header_2_5, text9, header_2_6, table1, header_2_7, text10,
    ];
}
