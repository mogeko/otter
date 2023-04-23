#[path = "../src/config.rs"]
mod config;

static CONFIG_FILE: &str = "./crates/core/examples/config/config.toml";

fn main() {
    println!("{:#?}", config::Config::new(CONFIG_FILE));
}
