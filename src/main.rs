#[macro_use]
extern crate clap;
extern crate docx_rs;
mod cmd;
mod components;
mod executor;
mod util;
use cmd::load::cli_build;

use std::fs::remove_dir;

use crate::executor::ssh::*;
use components::grafanainfo::grafana::*;
use components::sysinfo::system::*;
use executor::generator::gen_doc_tidb::*;

use docx_rs::*;
use util::table::*;

fn main() {
    let _ = cli_build();
    let ip_list = vec![
        SSHConfig {
            host: "139.155.15.210".to_string(),
            port: 7006,
            user: "tidb".to_string(),
            password: "tidb".to_string(),
            key_file: "".to_string(),
            passphrase: "".to_string(),
            timeout: 1000,
            exe_timeout: 1000,
        },
        SSHConfig {
            host: "139.155.15.210".to_string(),
            port: 7007,
            user: "tidb".to_string(),
            password: "tidb".to_string(),
            key_file: "".to_string(),
            passphrase: "".to_string(),
            timeout: 1000,
            exe_timeout: 1000,
        },
    ];

    let all_nodes_list = ClusterSSHHandle::new(&ip_list);
    // get system info from all address
    let cluster_nodes = ClusterSysInfo::new(&all_nodes_list);

    get_all_panel_image();
    // let sess = ssh.auth();
    // let mut remote_file = sess.scp_send(Path::new("1.txt"), 0o644, 10, None).unwrap();
    // remote_file.write(b"0123456789").unwrap();

    // let (mut remote_file, stat) = sess.scp_recv(Path::new("1.txt")).unwrap();
    // let mut contents = Vec::new();
    // remote_file.read_to_end(&mut contents).unwrap();
    // println!("{:?}", contents);

    let chapter = gen_chapter_system(&cluster_nodes);

    let mut dox = Docx::new();

    for elem in chapter.unwrap() {
        match elem {
            DocType::Patagraph(para) => dox = dox.add_paragraph(para),
            DocType::Table(tab) => dox = dox.add_table(tab),
        }
    }
    let _doc = gen_docx("./tidb_check.docx", &mut dox);

    let image_path = "/tmp/ticheck_image_dir".to_string();
    let _ = remove_dir(image_path);
}
