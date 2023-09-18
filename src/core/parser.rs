use super::input_buffer::InputBuffer;
use super::meta_commands::do_meta_command;

pub struct Parser {}

impl Parser {
    fn parse_statement() {
        println!("We are parsing statement");
    }

    pub fn parse(input_buffer: &InputBuffer) {
        match &input_buffer.buffer {
            Some(buffer) => {
                if buffer.chars().next() == Some('.') {
                    do_meta_command(&input_buffer);
                } else {
                    Parser::parse_statement();
                }
            },
            None => println!("Wrong way"),
        };
    }
}

