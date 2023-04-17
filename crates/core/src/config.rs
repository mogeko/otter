use config_rs::{ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
#[allow(unused)]
enum Proxy {
    HTTP(otter_http::Config<Proxy>),
    WG(otter_wg::Config),
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
    bind: Option<Vec<Proxy>>,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        config_rs::Config::builder()
            .add_source(File::with_name(path).required(false))
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        match Config::new("").unwrap().bind {
            None => assert!(true),
            _ => assert!(false),
        }
    }
}
