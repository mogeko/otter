use config_rs::{ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
#[allow(unused)]
enum Bind {
    HTTP(otter_http::InboundConfig<Tunnel>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
#[allow(unused)]
enum Tunnel {
    HTTP(otter_http::OutboundConfig),
    WG(otter_wg::OutboundConfig),
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
    #[serde(default)]
    binds: Vec<Bind>,
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
        Self { binds: vec![] }
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
