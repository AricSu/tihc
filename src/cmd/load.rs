use crate::components::grafanainfo::grafana::{get_all_images_count, get_all_panel_image};
use crate::components::sysinfo::system::ClusterSysInfo;
use crate::executor::generator::gen_doc_tidb::*;
use crate::executor::meta_parser;
use crate::executor::progress::terminal_pbr::*;
use crate::executor::ssh::{ClusterSSHHandle, SSHConfig};
use crate::util::time::*;
use anyhow::Result;
use clap::App;
use std::collections::HashMap;
use std::fs::remove_dir;
use std::sync::mpsc;
use std::thread;

use crate::util::table::*;
use docx_rs::*;

/// Build a cli using clap to process commands from terminal.
pub fn cli_build() -> Result<()> {
    // build clap structure from yaml and get matches from terminal.
    let yaml = load_yaml!("tihc_cmd.yml");
    let mut cli = App::from_yaml(yaml);
    let cli_matches = cli.clone().get_matches();

    // Config clap function for user entered wrong parameters;
    if let Some(_) = cli_matches.value_of("help") {
        let _ = cli.print_help();
    }

    // Print TiHC Version;
    if cli_matches.occurrences_of("version") == 1 {
        println!("TiHC Version : 1.0");
    }

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

        println!("TiHC Version : 1.0");
        println!("Starting TiDB Healthy Check from <TiHC>");
        println!("---------------------------------------");

        let mut vec_ssh: Vec<SSHConfig> = vec![];
        for host in distinct_host(meta_info.0, meta_info.1, meta_info.2).keys() {
            println!("Start getting node systeminfo from : --> {}", &host);
            vec_ssh.push(SSHConfig::new_auth_user(host.clone(), 22, ssh_user.to_owned(), ssh_pwd.to_owned()));
        }
        println!("Done getting all nodes systeminfo.");

        
        let all_nodes_list = ClusterSSHHandle::new(&vec_ssh);
        let cluster_nodes = ClusterSysInfo::new(&all_nodes_list);

        let grafana_start_time = Time::new()
            .from_string(grafana_start_time.to_string())
            .to_mills();
        let grafana_end_time = Time::new()
            .from_string(grafana_end_time.to_string())
            .to_mills();

        get_grafana_images(
            meta_info.5,
            meta_info.6,
            grafana_user.to_string(),
            grafana_pwd.to_string(),
            grafana_start_time,
            grafana_end_time,
        );

        get_tihc_doc(&cluster_nodes);
    };
    Ok(())
}

fn get_grafana_images(
    grafana_host: String,
    grafana_port: i64,
    grafana_user: String,
    grafana_pwd: String,
    grafana_start_time: i64,
    grafana_end_time: i64,
) {
    let format = "╢▌▌░╟".to_string();
    let header_str = "Start getting all imagenfo of grafana :".to_string();
    let finish_str = "Done getting all needed imagenfo of grafana.".to_string();
    let mut bar = Bar::new(header_str, format, true, finish_str, get_all_images_count());
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        bar.single_bar(rx);
    });

    let start_time = Time::new()
        .from_string(grafana_start_time.to_string())
        .to_mills();
    let end_time = Time::new()
        .from_string(grafana_end_time.to_string())
        .to_mills();

    get_all_panel_image(
        tx,
        grafana_user,
        grafana_pwd,
        grafana_host,
        grafana_port,
        start_time as u64,
        end_time as u64,
    );

    move_cursor_to_next_line();
}

fn get_tihc_doc(cluster_nodes: &ClusterSysInfo) {
    let format = "╢▌▌░╟".to_string();
    let header_str = "Start generating all chapters of healthy check output :".to_string();
    let finish_str = "Done generating all chapters of healthy check output.".to_string();
    let mut bar = Bar::new(header_str, format, true, finish_str, 7);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        bar.single_bar(rx);
    });
    gen_doc(tx, &cluster_nodes);

    move_cursor_to_next_line();
    println!("Successful exit to TiDB Healthy Check from <TiHC>");
    println!("-------------------------------------------------");
}

fn distinct_host(
    vec_tidb: Vec<(String, i64)>,
    vec_tikv: Vec<(String, i64)>,
    vec_pd: Vec<(String, i64)>,
) -> HashMap<String, (String, i64)> {
    let mut hash_map_tidb = distinct_hashmap(vec_tidb);
    let hash_map_tikv = distinct_hashmap(vec_tikv);
    let hash_map_pd = distinct_hashmap(vec_pd);

    hash_map_tidb.extend(hash_map_tikv);
    hash_map_tidb.extend(hash_map_pd);
    hash_map_tidb
}

fn distinct_hashmap(vec: Vec<(String, i64)>) -> HashMap<String, (String, i64)> {
    let mut host_map = HashMap::new();
    for host_unit in vec {
        if !host_map.contains_key(&host_unit.0) {
            host_map.insert(host_unit.0.clone(), host_unit);
        }
    }
    host_map
}

fn gen_doc(tx: mpsc::Sender<u64>, cluster_nodes: &ClusterSysInfo) {
    let chapter = gen_chapter_system(&cluster_nodes);

    let mut dox = Docx::new();

    let mut progress = 0;

    for elem in chapter.unwrap() {
        match elem {
            DocType::Patagraph(para) => dox = dox.add_paragraph(para),
            DocType::Table(tab) => dox = dox.add_table(tab),
        }
        progress = progress + 1;
        tx.send(progress).unwrap();
    }
    let _doc = gen_docx("./tidb_check.docx", &mut dox);

    let image_path = "/tmp/ticheck_image_dir".to_string();
    let _ = remove_dir(image_path);
}
