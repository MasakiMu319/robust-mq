use crate::tools::read_file;
use serde::Deserialize;
use std::sync::OnceLock;
use toml::Table;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PlacementCenterConfig {
    pub cluster_name: String,
    pub addr: String,
    #[serde(default = "default_node_id")]
    pub node_id: u64,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: usize,
    pub nodes: Table,
    pub http_port: usize,
    pub data_path: String,
    pub log: Log,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Log {
    pub log_config: String,
    pub log_path: String,
}

pub fn default_node_id() -> u64 {
    1
}

pub fn default_grpc_port() -> usize {
    50050
}

static PLACEMENT_CENTER_CONF: OnceLock<PlacementCenterConfig> = OnceLock::new();

pub fn init_placement_center_conf_by_path(config_path: &String) -> &'static PlacementCenterConfig {
    PLACEMENT_CENTER_CONF.get_or_init(|| {
        let content = read_file(config_path).unwrap();
        let pc_config: PlacementCenterConfig = toml::from_str(&content).unwrap();
        return pc_config;
    })
}

pub fn placement_center_conf() -> &'static PlacementCenterConfig {
    match PLACEMENT_CENTER_CONF.get() {
        Some(conf) => {
            return conf;
        }
        None => {
            panic!(
                "Placement center configuration is not initialized, check the configuration file."
            );
        }
    }
}
