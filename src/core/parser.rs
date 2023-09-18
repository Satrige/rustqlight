use super::input_buffer::InputBuffer;
use super::meta_commands::{ do_meta_command, MetaCommandResult };
use super::statements::{prepare_statement, PrepareResult};

pub fn parse(input_buffer: &InputBuffer) {
    match &input_buffer.buffer {
        Some(buffer) => {
            if buffer.chars().next() == Some('.') {
                match do_meta_command(&input_buffer) {
                    MetaCommandResult::MetaCommandSuccess => {
                        println!("Success!");
                    },
                    MetaCommandResult::MetaCommandUnrecognizedCommand => {
                        println!("Unrecognized command: {:}", buffer);
                    }
                }
            } else {
                match prepare_statement(&input_buffer) {
                    PrepareResult::PrepareSuccess => {
                        println!("Prepared statement wasn't implementef yet");
                    },
                    PrepareResult::PrepareUnrecognizedStatement => {
                        println!("Unrecognized statment here: {:}", buffer);
                    }
                }
            }
        },
        None => println!("Wrong way"),
    };
}
