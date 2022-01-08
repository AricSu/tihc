extern crate docx_rs;
use crate::components::sysinfo::system::*;
use crate::executor::generator::gen_doc_backup::*;
use crate::executor::generator::gen_doc_database::*;
use crate::executor::generator::gen_doc_disater::*;
use crate::executor::generator::gen_doc_introduce::*;
use crate::executor::generator::gen_doc_sys::*;
use crate::util::table::*;

pub fn gen_chapter_system(cluster_nodes: &ClusterSysInfo) -> anyhow::Result<Vec<DocType>> {
    let mut chaper_system_elment = vec![];
    chaper_system_elment.append(&mut gen_chapter_system_1());
    chaper_system_elment.append(&mut gen_chapter_system_2());
    chaper_system_elment.append(&mut gen_chapter_system_3(cluster_nodes));
    chaper_system_elment.append(&mut gen_chapter_system_4()?);
    chaper_system_elment.append(&mut gen_chapter_system_5());
    chaper_system_elment.append(&mut gen_chapter_system_6());
    chaper_system_elment.append(&mut gen_chapter_system_7());

    Ok(chaper_system_elment)
}
