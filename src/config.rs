use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::default::Default;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    listen_addr: String,
    unknown_host_message: String,
    hosts: BTreeMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:25565".to_string(),
            unknown_host_message: "{\"text\":\"Invalid address.\",\"color\":\"red\"}".to_string(),
            hosts: BTreeMap::new(),
        }
    }
}

impl Config {
    pub fn load_or_init(path: &Path) -> Config {
        if path.exists() {
            toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
        } else {
            info!("Configuration file does not exist. Use defaults.");
            let default = Config::default();
            trace!("Default configuration: {:?}", default);
            let string = toml::to_string(&default).unwrap();
            fs::write(path, &string).unwrap();
            default
        }
    }

    pub fn get_unknown_host_message(&self) -> &str {
        &self.unknown_host_message
    }

    pub fn get_listen_addr(&self) -> &str {
        &self.listen_addr
    }

    pub fn get_hosts(&self) -> &BTreeMap<String, String> {
        &self.hosts
    }

    pub fn get_addr_by_host(&self, host: &str) -> Option<&String> {
        self.hosts.get(host)
    }
}
