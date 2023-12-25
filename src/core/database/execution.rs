use crate::core::{
    enums::Statements,
    types::parsed_statement::ParsedStatement,
};

use super::statements;
use super::meta_commands;

pub fn execute(parsed_statement: &ParsedStatement) {
    match parsed_statement.statement_type {
        Statements::Statement => {
            statements::execute(&parsed_statement.statement.as_ref().unwrap());
        },
        Statements::MetaCommand => {
            meta_commands::execute(&parsed_statement.meta_command);
        },
    }
}