use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub client: ClientConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub client_addr: String,
    pub user_addr: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientConfig {
    pub server_addr: String,
    pub target_addr: String,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path.as_ref().to_str().unwrap()))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
} 