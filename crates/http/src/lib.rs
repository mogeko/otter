use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct InboundConfig<T> {
    tunnels: Vec<T>,
    addr: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct OutboundConfig {
    addr: String,
    port: u16,
}
