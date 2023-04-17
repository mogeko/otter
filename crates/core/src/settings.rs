use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
#[allow(unused)]
enum Proxy {
    HTTP(otter_http::Settings<Proxy>),
    WG(otter_wg::Settings),
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    bind: Option<Vec<Proxy>>,
}

impl Settings {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        Config::builder()
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
        match Settings::new("").unwrap().bind {
            None => assert!(true),
            _ => assert!(false),
        }
    }
}
