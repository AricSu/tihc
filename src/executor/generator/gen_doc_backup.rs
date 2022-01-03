extern crate docx_rs;
use crate::util::table::*;
use docx_rs::*;

pub fn gen_chapter_system_6() -> Vec<DocType> {
    let header_6 = DocType::Patagraph(gen_heading("六、备份与恢复", 40, 1));
    let text1 = DocType::Patagraph(gen_text(
        "    备份采用 <dumpling 或 BR> 方式进行备份，观察备份日志输出判断目前备份正常。",
        20,
        "Red",
    ));
    let table_header =
        gen_table_header(vec!["备份内容", "备份方式", "备份目录", "备份成功", "备注"]);

    // generate table rows
    let row = gen_table_row(
        vec![
            &mut "全库备份".to_string(),
            &mut "dumpling".to_string(),
            &mut "/backup/path".to_string(),
            &mut "".to_string(),
            &mut "".to_string(),
        ],
        20,
        "red",
    );

    let table = gen_table(
        table_header,
        &mut vec![row],
        vec![1000, 1000, 3000, 1000, 1000],
        TableLayoutType::Fixed,
        500,
    );
    let table1 = DocType::Table(table);
    return vec![header_6, text1, table1];
}
