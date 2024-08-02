mod process;
mod parser;
mod enums;
mod database;
mod statement;

use process::Process;

fn main() {
    let mut process = Process::new();

    process.run();
}
