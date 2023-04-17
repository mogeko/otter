#[path = "../src/settings.rs"]
#[allow(unused)]
mod settings;

use settings::Settings;

static CONFIG_FILE: &str = "./crates/core/examples/etc/otter/config.json";

fn main() {
    println!("{:#?}", Settings::new(CONFIG_FILE));
}
