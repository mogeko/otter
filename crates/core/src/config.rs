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
#[allow(unused)]
pub struct Config {
    #[serde(default)]
    listener: Vec<Listener>,
    endpoint: Vec<Endpoint>,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        config_rs::Config::builder()
            .add_source(File::with_name(path).required(false))
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
