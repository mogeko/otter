use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct EndpointConfig {
    addr: String,
    port: u16,
    priv_key: String,
    pub_key: String,
}
