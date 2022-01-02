extern crate docx_rs;
mod components;
mod executor;
mod util;

use crate::executor::ssh::*;
use components::sysinfo::system::*;
use executor::generator::gen_doc_sys::*;

use crate::util::table::*;
use crate::util::time::*;
use docx_rs::*;
use std::fs::create_dir_all;
use std::path::Path;

use chrono::{DateTime, Utc};
use std::fs::*;
use std::io::Read;
use std::process::Command;

fn main() {
    // let sess = ssh.auth();
    // let mut remote_file = sess.scp_send(Path::new("1.txt"), 0o644, 10, None).unwrap();
    // remote_file.write(b"0123456789").unwrap();

    // let (mut remote_file, stat) = sess.scp_recv(Path::new("1.txt")).unwrap();
    // let mut contents = Vec::new();
    // remote_file.read_to_end(&mut contents).unwrap();
    // println!("{:?}", contents);
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

    // curl -o blah.png http://admin:admin@127.0.0.1:3000/render/d-solo/eDbRZpnWk/playground-overview\?orgId=1\&from\=1641136649222\&to\=1641136949222\&refresh\=30s\&panelId\=34\&width\=1000\&height\=500\&tz\=Asia%2FShanghai
    let start_time: DateTime<Utc> = Utc::now();
    let hash_time = calculate_hash(&start_time);
    let tmp_addr = format!("/tmp/ticheck_grafana_image_{}", hash_time);

    let path = Path::new(&tmp_addr);
    match create_dir_all(path) {
        Ok(_f) => {
            println!("created folder")
        }
        Err(err) => {
            println!("{:?}", err);
        }
    };

    let cmd_str = format!("curl -o {}/02_SQL_Duration.png http://admin:admin@127.0.0.1:3000/render/d-solo/eDbRZpnWk/playground-overview\\?orgId=1\\&from\\=1641136649222\\&to\\=1641136949222\\&refresh\\=30s\\&panelId\\=34\\&width\\=1000\\&height\\=500\\&tz\\=Asia%2FShanghai", tmp_addr);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd_str)
        .output()
        .expect("sh exec error!");
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);

    let mut img = File::open(format!("{}/02_SQL_Duration.png", &tmp_addr)).unwrap();
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf).unwrap();

    let pic = Pic::new(buf).size(500, 250);

    let chapter = gen_chapter_system(&cluster_nodes);

    let mut dox = Docx::new()
    .add_paragraph(gen_heading("一、检查介绍", 40, 1))
    .add_paragraph(gen_heading("1.1 检查系统", 30, 2))
    .add_paragraph(gen_text("   本次检查数据库为 <yyyy> 生产库系统。"))
    .add_paragraph(gen_text("   本报告提供的检查和建议不涉及具体的数据库安全分析和应用程序细节。本次数据库涉及了 1 套 <N> 节点TiDB数据库的检查，在这次检查中对主机和数据库配置和数据库性能进行了总体分析，不针对具体某个应用性能。"))
    .add_paragraph(gen_heading("1.2 检查方法", 30, 2))
    .add_paragraph(gen_text("  本次数据库性能检查的工具是："))
    .add_paragraph(gen_text("  上述输出结果为建议提供依据。"))
    .add_paragraph(gen_heading("1.3 检查范围", 30, 2))
    .add_paragraph(gen_text("  本次检查数据库为 <yyyy> 生产库系统，涉及了 1 套 <N> 节点TiDB数据库的检查，在这次检查中对主机和数据库配置和数据库性能进行了总体分析，不针对具体某个应用性能。"))
    .add_paragraph(gen_text("  本报告提供的检查和建议不涉及具体的数据库安全分析和应用程序细节。"))
    .add_paragraph(gen_heading("二、检查总结", 40, 1))
    .add_paragraph(gen_heading("2.1 操作系统配置建议", 30, 2))
    .add_paragraph(gen_heading("2.2 数据库版本建议", 30, 2))
    .add_paragraph(gen_heading("2.3 数据库参数建议", 30, 2))
    .add_paragraph(gen_heading("2.4 数据库组件建议", 30, 2))
    .add_paragraph(gen_heading("2.5 数据库日志建议", 30, 2))
    .add_paragraph(gen_heading("2.6 数据库管理建议", 30, 2))
    .add_paragraph(gen_heading("2.7 数据库备份建议", 30, 2))
    .add_paragraph(gen_heading("三、系统配置", 40, 1))
    .add_paragraph(gen_heading("3.2 操作系统", 30, 2))
    .add_paragraph(gen_heading("3.2.1 操作系统版本信息", 20, 3))
    // .add_table(system_table)
    .add_paragraph(gen_heading("3.2.2 内核参数配置信息", 20, 3))
    .add_paragraph(gen_heading("3.2.3 Ulimit 参数配置信息", 20, 3))
    .add_paragraph(gen_heading("3.2.4 Swap 状态信息", 20, 3))
    .add_paragraph(gen_heading("3.2.5 磁盘调度策略信息", 20, 3))
    .add_paragraph(gen_heading("3.2.6 透明大页配置信息", 20, 3))
    .add_paragraph(gen_heading("3.2.7 NTP服务状态信息", 20, 3))
    .add_paragraph(gen_heading("3.2.8 磁盘挂载参数信息", 20, 3))
    .add_paragraph(gen_heading("3.2.9 防火墙运行状态信息", 20, 3))
    .add_paragraph(gen_heading("3.2.10 CPU 运行模式信息", 20, 3))
    .add_paragraph(gen_heading("3.3 网络配置", 30, 2))
    .add_paragraph(gen_heading("3.4 设备概况", 30, 2))
    .add_paragraph(gen_heading("四、数据库集群配置", 40, 1))
    .add_paragraph(gen_heading("4.1 TiDB系统service清单", 30, 2))
    .add_paragraph(gen_heading("4.2 组件清单配置", 30, 2))
    .add_paragraph(gen_heading("4.3 数据库配置", 30, 2))
    .add_paragraph(gen_heading("4.3.1 软件版本", 20, 3))
    .add_paragraph(gen_heading("4.3.2 数据库参数", 20, 3))
    .add_paragraph(gen_heading("4.4 集群概览", 30, 2))
    .add_paragraph(gen_heading("4.4.1 PD概览", 20, 3))
    .add_paragraph(gen_heading("4.4.2 TiDB概览", 20, 3))
    .add_paragraph(Paragraph::new().add_run(Run::new().add_image(pic)))
    .add_paragraph(gen_heading("4.4.3 TiKV概览", 20, 3))
    .add_paragraph(gen_heading("4.4.4 系统信息概览", 20, 3))
    .add_paragraph(gen_heading("4.5 数据库日志", 30, 2))
    .add_paragraph(gen_heading("五、数据库性能", 40, 1))
    .add_paragraph(gen_heading("5.1 SQL性能概况", 30, 2))
    .add_paragraph(gen_heading("5.2 慢 SQL 1 根因分析", 30, 2))
    .add_paragraph(gen_heading("5.3 慢 SQL 2 根因分析", 30, 2))
    .add_paragraph(gen_heading("六、备份与恢复", 40, 1))
    .add_paragraph(gen_heading("七、容灾与高可用评估", 40, 1));

    // for elem in chapter {
    //     match elem {
    //         DocType::Patagraph(para) => dox = dox.add_paragraph(para),
    //         DocType::Table(tab) => dox = dox.add_table(tab),
    //     }
    // }
    let _doc = gen_docx("./tidb_check.docx", &mut dox);
}
