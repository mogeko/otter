#[path = "../src/config.rs"]
mod config;

static CONFIG_FILE: &str = "./crates/core/examples/etc/otter/config.json";

fn main() {
    println!("{:#?}", config::Config::new(CONFIG_FILE));
}
