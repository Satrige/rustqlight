use super::input_buffer::InputBuffer;
use super::meta_commands::{ do_meta_command, MetaCommandResult };
use super::statements::{ prepare_statement, Statement };

pub fn parse(input_buffer: &InputBuffer) -> Option<Box<dyn Statement>> {
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

                return None;
            } else {
                return prepare_statement(&input_buffer);
            }
        },
        None => {
            println!("Wrong way");

            return None;
        }
    };
}
