mod database;
mod db_config;
mod db_loader;
mod table;

use clap::Parser;
use database::Database;
use db_config::DbConfig;
use env_logger;

#[derive(Parser, Debug)]
struct Opts {
    #[arg(short, long, help = "Path to the config file")]
    config: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let opts = Opts::parse();

    let config = match opts.config {
        Some(config_path) => DbConfig::parse_config_file(&config_path).unwrap(),
        None => DbConfig::new(),
    };

    let database = Database::new(&config).unwrap();

    database.load();
}
