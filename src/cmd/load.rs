use crate::components::grafanainfo::grafana::get_all_panel_image;
use crate::components::sysinfo::system::ClusterSysInfo;
use crate::executor::generator::gen_doc_tidb::*;
use crate::executor::meta_parser;
use crate::executor::ssh::{get_key_file_path, ClusterSSHHandle, SSHConfig};
use anyhow::Result;
use clap::App;
use std::fs::remove_dir;
use std::str::FromStr;

use crate::util::table::*;
use docx_rs::*;

/// Match commands
pub fn cli_build() -> Result<()> {
    // Get matches
    let yaml = load_yaml!("tihc_cmd.yml");
    let mut cli = App::from_yaml(yaml);

    // config clap function for user entered wrong parameters;
    let _ = cli.print_help();
    let cli_matches = cli.get_matches();

    if let (
        Some(cluster_name),
        Some(grafana_user),
        Some(grafana_pwd),
        Some(grafana_start_time),
        Some(grafana_end_time),
        Some(ssh_user),
        Some(ssh_pwd),
    ) = (
        cli_matches.value_of("cluster_name"),
        cli_matches.value_of("grafana_user"),
        cli_matches.value_of("grafana_pwd"),
        cli_matches.value_of("grafana_start_time"),
        cli_matches.value_of("grafana_end_time"),
        cli_matches.value_of("ssh_user"),
        cli_matches.value_of("ssh_pwd"),
    ) {
        let cluster_name_string = cluster_name.to_string();
        let meta_info = meta_parser::init(cluster_name_string.clone());

        // get ssh key file path
        let ssh_key_file = get_key_file_path(cluster_name_string);

        let mut vec_ssh: Vec<SSHConfig> = vec![];
        for idx in 1..=meta_info.0.len() {
            vec_ssh.append(&mut vec![SSHConfig::new_auth_user(
                meta_info.0[idx].clone(),
                22,
                ssh_user.to_string(),
                ssh_pwd.to_string(),
            )]);
        }
        let all_nodes_list = ClusterSSHHandle::new(&vec_ssh);
        let cluster_nodes = ClusterSysInfo::new(&all_nodes_list);

        get_all_panel_image(
            grafana_user.to_string(),
            grafana_pwd.to_string(),
            meta_info.3 .2,
            meta_info.3 .3,
            u64::from_str(grafana_start_time).unwrap(),
            u64::from_str(grafana_end_time).unwrap(),
        );

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
    };

    //     let ip_list = vec![
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

    // else {
    //     // let meta_handle = meta_parser::init();
    //    // let _ = SSHConfig::new_auth_user(host, port, user.to_string(), pwd.to_string());
    // }

    Ok(())
}
