mod database;

use clap::{Parser};

#[derive(Parser, Debug)]
struct Opts {
    #[arg(short, long, help = "Path to the config file", default_value = "./config.yml")]
    config: String,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
}
