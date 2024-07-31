use crate::core::enums::Statements;
use super::statement::Statement;

pub struct ParsedStatement {
    pub statement_type: Statements,

    pub statement: Option<Statement>,
    pub meta_command: Option<String>,
}
