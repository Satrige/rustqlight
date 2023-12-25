use crate::core::enums::Statements;
use super::statement::Statement;

pub struct ParsedStatement {
    pub statement_type: Statements,

    pub statement: Option<Box<Statement>>,
    pub meta_command: Option<Box<String>>,
}
