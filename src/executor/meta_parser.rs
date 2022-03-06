use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{yaml, Yaml};

// const META_PATH: &str = "~/.tiup/storage/cluster/clusters/{}";

const META_PATH: &str = "/Users/suzhipeng/Database/TiHC/tihc/";

pub fn init(
    cluster_name: String,
) -> (
    Vec<(String, i64)>,
    Vec<(String, i64)>,
    Vec<(String, i64)>,
    (String, String, String, i64),
) {
    let meta = HCYmal::new(format!("{}{}", META_PATH, "meta.yaml"));
    let tidb_node_host = meta.clone().get_host_and_port("tidb_servers".to_string());
    let tikv_node_host = meta.clone().get_host_and_port("tikv_servers".to_string());
    let pd_node_host = meta.clone().get_host_and_port("pd_servers".to_string());
    let grafana_info = meta.get_grafana_sign_info();
    return (tidb_node_host, tikv_node_host, pd_node_host, grafana_info);
}

#[derive(Debug, Clone)]
pub struct HCYmal {
    file_path: String,
    meta_handle: Vec<Yaml>,
}

impl HCYmal {
    pub fn new(path: String) -> Self {
        let mut f = File::open(format!("{}{}", META_PATH, "meta.yaml")).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let vec_ymal = yaml::YamlLoader::load_from_str(&s).unwrap();
        return {
            HCYmal {
                file_path: path,
                meta_handle: vec_ymal,
            }
        };
    }
    fn get_host_and_port(self, component_type: String) -> Vec<(String, i64)> {
        let mut component_host: Vec<(String, i64)> = vec![];
        for doc in self.meta_handle {
            for topo_sub_item in doc["topology"][component_type.as_str()].as_vec().unwrap() {
                // println!("{}", topo_sub_item["host"].as_str().unwrap());
                // println!("{}", topo_sub_item["ssh_port"].clone().into_i64().unwrap());
                component_host.append(&mut vec![(
                    topo_sub_item["host"].clone().into_string().unwrap(),
                    topo_sub_item["ssh_port"].clone().into_i64().unwrap(),
                )]);
            }
        }
        component_host
    }

    fn get_grafana_sign_info(self) -> (String, String, String, i64) {
        let mut grafana_user: String = "".to_string();
        let mut grafana_pwd: String = "".to_string();
        let mut grafana_host: String = "".to_string();
        let mut grafana_port: i64 = 3000;
        for doc in self.meta_handle {
            for topo_sub_item in doc["topology"]["grafana_servers"].as_vec().unwrap() {
                grafana_host = topo_sub_item["host"].clone().into_string().unwrap();
                grafana_port = topo_sub_item["port"].as_i64().unwrap();
                grafana_user = topo_sub_item["username"].clone().into_string().unwrap();
                grafana_pwd = topo_sub_item["password"].clone().into_string().unwrap();
            }
        }
        (grafana_user, grafana_pwd, grafana_host, grafana_port)
    }
}
