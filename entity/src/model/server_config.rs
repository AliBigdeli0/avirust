use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    pub listen: String,
    pub port: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub server: ServerInfo,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            listen: String::from("0.0.0.0"),
            port: 9001
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server: Default::default()
        }
    }
}