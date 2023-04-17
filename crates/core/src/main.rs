mod settings;

use clap::Parser;

/// A flexible and efficient HTTP proxy
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "/etc/otter/config")]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _config = settings::Settings::new(&args.config).unwrap();

    println!("{}", otter::hello());
}
