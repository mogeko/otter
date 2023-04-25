use config_rs::{ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
#[allow(unused)]
enum Listener {
    HTTP(otter_http::ListenerConfig),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
#[allow(unused)]
enum Endpoint {
    HTTP(otter_http::EndpointConfig),
    WG(otter_wg::EndpointConfig),
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[allow(unused)]
pub struct Config {
    listener: Vec<Listener>,
    endpoint: Vec<Endpoint>,
}

impl Config {
    pub fn new(_path: &str) -> Result<Self, ConfigError> {
        let global_config = std::path::Path::new("/etc/otter/config.toml");
        let home_config = dirs::home_dir().unwrap().join(".config/otter/config.toml");
        let user_config = std::path::Path::new(_path);

        config_rs::Config::builder()
            .add_source(File::from(global_config).required(false))
            .add_source(File::from(home_config).required(false))
            .add_source(File::from(user_config).required(false))
            .build()?
            .try_deserialize()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listener: vec![],
            endpoint: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Config::new("").is_ok());
    }
}
