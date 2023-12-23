use super::prompt;
use crate::core::types::parsed_statement::ParsedStatement;

pub fn parse_with_prompt() -> Option<ParsedStatement> {
    let input_buffer = prompt::get_input();

    match &input_buffer.buffer {
        Some(buffer) => {
            return Some(ParsedStatement::new(buffer)); 
        },
        None => {
            println!("Wrong way");

            return None;
        }
    };
}
