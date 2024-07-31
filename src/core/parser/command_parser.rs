use super::prompt;
use crate::core::{
    enums::Statements,
    types::parsed_statement::ParsedStatement,
};

pub fn parse_with_prompt() -> Option<ParsedStatement> {
    let input_buffer = prompt::get_input();

    match &input_buffer.buffer {
        Some(buffer) => {
            let parsed_statement = ParsedStatement::new(buffer);
            if parsed_statement.statement_type == Statements::Statement {
                match &parsed_statement.statement {
                    Some(_) => {
                        return Some(parsed_statement);
                    },
                    None => {
                        return None;
                    },
                }
            } else {
                return Some(parsed_statement);
            }
        },
        None => {
            println!("Wrong way");

            return None;
        }
    };
}
