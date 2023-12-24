use std::process;

pub fn execute(parsed_meta_command: &Option<Box<String>>) {
    let meta_command = parsed_meta_command.as_ref().unwrap();

    match (&**meta_command).as_str() {
        ".exit" => {
            process::exit(0);
        },
        _ => {
            println!("The metacommand was not implemented yet");
        }
    }
}
