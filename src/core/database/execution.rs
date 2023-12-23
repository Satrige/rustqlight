use crate::core::{
    enums::Statements,
    types::parsed_statement::ParsedStatement,
};
use super::meta_commands;

pub fn execute(parsed_statement: &ParsedStatement) {
    match parsed_statement.statement_type {
        Statements::MetaCommand => {
            meta_commands::execute(parsed_meta_command)
        },
        Statements::Statement => {

        }
    }
}