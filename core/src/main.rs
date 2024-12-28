use clap::{Parser};

#[derive(Parser, Debug)]
struct Opts {
    #[arg(short, long, help = "Path to the config file")]
    config: String,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    println!("{:#?}", opts);
}
