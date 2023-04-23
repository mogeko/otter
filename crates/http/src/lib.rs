use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct InboundConfig {
    addr: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct OutboundConfig {
    addr: String,
    port: u16,
}
