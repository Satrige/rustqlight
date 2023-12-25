mod core;

fn main() {
    let mut process = core::types::process::Process::new();

    process.run();
}
