#[macro_use]
extern crate clap;
extern crate colored;
extern crate docx_rs;
extern crate pbr;
extern crate yaml_rust;
mod cmd;
mod components;
mod executor;
mod util;
use cmd::load::cli_build;

use std::fs::remove_dir;

use crate::executor::ssh::*;
use components::sysinfo::system::*;
use executor::generator::gen_doc_tidb::*;

use docx_rs::*;
use util::table::*;

fn main() {
    let _ = cli_build();

    // let ip_list = vec![
    //     SSHConfig {
    //         host: "139.155.15.210".to_string(),
    //         port: 7006,
    //         user: "tidb".to_string(),
    //         password: "tidb".to_string(),
    //         key_file: "".to_string(),
    //     },
    //     SSHConfig {
    //         host: "139.155.15.210".to_string(),
    //         port: 7007,
    //         user: "tidb".to_string(),
    //         password: "tidb".to_string(),
    //         key_file: "".to_string(),
    //     },
    // ];

    // let all_nodes_list = ClusterSSHHandle::new(&ip_list);
    // // get system info from all address
    // let cluster_nodes = ClusterSysInfo::new(&all_nodes_list);

    // get_all_panel_image();

    // let chapter = gen_chapter_system(&cluster_nodes);

    // let mut dox = Docx::new();

    // for elem in chapter.unwrap() {
    //     match elem {
    //         DocType::Patagraph(para) => dox = dox.add_paragraph(para),
    //         DocType::Table(tab) => dox = dox.add_table(tab),
    //     }
    // }
    // let _doc = gen_docx("./tidb_check.docx", &mut dox);

    // let image_path = "/tmp/ticheck_image_dir".to_string();
    // let _ = remove_dir(image_path);
}
