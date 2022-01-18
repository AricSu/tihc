use crate::executor::ssh::ClusterSSHHandle;
use ssh2::Session;
use std::io::prelude::*;

pub struct SysInfo {
    pub sys_host: String,
    pub sys_version: String,
    pub kernel_version: String,
    pub sys_limit: String,
    pub sys_conf: String,
    pub swap_status: String,
    pub ntp_status: String,
    pub thp_status: String,
    pub disk_strategy: String,
    pub disk_mount: String,
    pub disk_type: String,
    pub firewalld_status: String,
    pub cpu_mode: String,
    pub numa_status: String,
    pub bond_status: String,
}

impl SysInfo {
    pub fn new(session: &Session) -> Self {
        SysInfo {
            sys_host: get_sys_host(session),
            sys_version: get_sys_version(session),
            kernel_version: get_kernel_version(session),
            sys_limit: get_sys_limit(session),
            sys_conf: get_sys_conf(session),
            swap_status: get_swap_status(session),
            ntp_status: get_ntp_status(session),
            thp_status: get_thp_status(session),
            disk_strategy: get_disk_strategy(session),
            disk_mount: get_disk_mount(session),
            disk_type: get_disk_type(session),
            firewalld_status: get_firewalld_status(session),
            cpu_mode: get_cpu_mode(session),
            numa_status: get_numa_status(session),
            bond_status: get_ethcard_bond(session),
        }
    }
}

fn get_sys_host(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel
        .exec("cat /etc/sysconfig/network-scripts/ifcfg-* |grep IPADDR|grep -v 127")
        .unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_kernel_version(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("uname -a").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_sys_version(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("cat /etc/redhat-release").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_disk_type(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("cat /etc/redhat-release").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

pub fn get_sys_limit(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel
        .exec("cat /etc/security/limits.conf|grep -v '#'")
        .unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_sys_conf(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel
        .exec("cat /etc/sysctl.conf|grep -v '#'")
        .unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_swap_status(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("free -g|grep Swap").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_ntp_status(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("ntpstat").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_disk_strategy(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("mount -t ext4 ").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_thp_status(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel
        .exec("cat /sys/kernel/mm/transparent_hugepage/enabled")
        .unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_disk_mount(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("free -g|grep Swap").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_firewalld_status(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel
        .exec("systemctl status firewalld |grep Active")
        .unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_cpu_mode(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel
        .exec("cpupower frequency-info --policy")
        .unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_ethcard_bond(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("ip a").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    return s;
}

fn get_numa_status(ssh_session: &Session) -> String {
    let mut get_channel = ssh_session.channel_session().unwrap();
    get_channel.exec("echo numa").unwrap();
    let mut s = String::new();
    get_channel.read_to_string(&mut s).unwrap();
    // TODO :: why close?
    // get_channel.wait_close();
    // println!("{}", get_channel.exit_status().unwrap());
    return s;
}

pub struct ClusterSysInfo {
    pub all_nodes: Vec<SysInfo>,
}

impl ClusterSysInfo {
    pub fn new(all_nodes_ssh: &ClusterSSHHandle) -> Self {
        let mut all_node_sys_info = vec![];

        for ssh_hanler in &all_nodes_ssh.all_handler {
            let node_info = SysInfo::new(&ssh_hanler);
            all_node_sys_info.append(&mut vec![node_info]);
        }

        return ClusterSysInfo {
            all_nodes: all_node_sys_info,
        };
    }
}
