extern crate docx_rs;
use crate::util::table::*;

pub fn gen_chapter_system_7() -> Vec<DocType> {
    let header_7 = DocType::Patagraph(gen_heading("七、容灾与高可用评估", 40, 1));
    let text1 = DocType::Patagraph(gen_text(
        "    <系统为主生产中心和同城备生产中心，另在异地有数据级备份系统>",
        20,
        "Red",
    ));
    return vec![header_7, text1];
}
