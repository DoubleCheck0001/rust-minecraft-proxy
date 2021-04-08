use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::default::Default;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MOTD {
    text: String,
    protocol_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub(crate) ip: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownHost {
    kick_message: String,
    motd: MOTD
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    listen_addr: String,
    unknown_host: UnknownHost,
    hosts: BTreeMap<String, Server>
}

impl Default for Config {
    fn default() -> Self {

        let unknown_host = UnknownHost {
            kick_message: "§bRust Minecraft Proxy\n\n§cInvalid Address".to_string(),
            motd: MOTD { text: "§cUnknown host!\n§7Please use a valid address to connect.".to_string(), protocol_name: "§crust-minecraft-proxy".to_string() }
        };

        let mut hosts: BTreeMap<String, Server> = BTreeMap::new();
        hosts.insert("hub.example.com".to_string(), Server { ip: "127.0.0.1:35560".to_string() });
        hosts.insert("minigame.example.com".to_string(), Server { ip: "127.0.0.1:25561".to_string() });

        Self {
            listen_addr: "0.0.0.0:25565".to_string(),
            unknown_host,
            hosts
        }
    }
}

impl Config {
    pub fn load_or_init(path: &Path) -> Config {
        if path.exists() {
            serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
        } else {
            info!("Configuration file does not exist. Use defaults.");
            let default = Config::default();
            trace!("Default configuration: {:?}", default);
            let string = serde_yaml::to_string(&default).unwrap();
            fs::write(path, &string).unwrap();
            default
        }
    }

    pub fn get_unknown_host_kick_msg(&self) -> String {
        let mut message: String =  "{\"text\":\"".to_owned();
        message.push_str(&self.unknown_host.kick_message);
        message.push_str("\"}");
        message
    }

    pub fn get_unknown_host_motd(&self) -> String {
        let mut motd: String = "{\"version\": {\"name\": \"".to_owned();
        motd.push_str(&self.unknown_host.motd.protocol_name);
        motd.push_str("\", \"protocol\": -1 }, \"players\": {\"max\": 0, \"online\": 0, \"sample\": [] }, \"description\": { \"text\": \"");
        motd.push_str(&self.unknown_host.motd.text);
        motd.push_str("\" }}");
        motd
    }

    pub fn get_listen_addr(&self) -> &str {
        &self.listen_addr
    }

    pub fn get_hosts(&self) -> &BTreeMap<String, Server> {
        &self.hosts
    }

    pub fn get_addr_by_host(&self, host: &str) -> Option<&Server> {
        self.hosts.get(host)
    }
}
