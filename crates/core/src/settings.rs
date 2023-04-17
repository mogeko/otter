use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
#[allow(unused)]
enum Proxy {
    HTTP(HTTP),
    WG(otter_wg::Config),
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct HTTP {
    tunnel: Option<Vec<Proxy>>,
    addr: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    bind: Option<Vec<Proxy>>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Settings::from("/etc/otter/config")
    }

    pub fn from(path: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(path).required(false))
            .build()?
            .try_deserialize()
    }
}
