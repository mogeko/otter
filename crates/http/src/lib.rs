use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ListenerConfig {
    addr: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct EndpointConfig {
    addr: String,
    port: u16,
}
