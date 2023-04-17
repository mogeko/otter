use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings<T> {
    tunnel: Option<Vec<T>>,
    addr: String,
    port: u16,
}
