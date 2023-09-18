use std::process::exit;
use super::input_buffer::InputBuffer;

pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult  {
    let command = match &input_buffer.buffer {
        Some(command) => { (*command).as_str() },
        None => {
            return MetaCommandResult::MetaCommandUnrecognizedCommand;
        }
    };

    if command == ".exit".to_string() {
        exit(0);
    } else {
        return MetaCommandResult::MetaCommandUnrecognizedCommand;
    }
}