use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config<T> {
    tunnel: Option<Vec<T>>,
    addr: String,
    port: u16,
}
