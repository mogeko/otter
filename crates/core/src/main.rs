mod settings;

use settings::Settings;

#[tokio::main]
async fn main() {
    println!("{}", otter::hello());

    println!("{:?}", Settings::new());
}
