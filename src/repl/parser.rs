use crate::common::types::input_buffer::InputBuffer;

pub struct Parser {}

impl Parser {
    fn parse_meta_command() {
        println!("We are parsing meta command");
    }

    fn parse_statement() {
        println!("We are parsing statement");
    }

    pub fn parse(input_buffer: &InputBuffer) {
        match &input_buffer.buffer {
            Some(buffer) => {
                if buffer.chars().next() == Some('.') {
                    Parser::parse_meta_command();
                } else {
                    Parser::parse_statement();
                }
            },
            None => println!("Wrong way"),
        };
    }
}

