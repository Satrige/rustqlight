use crate::core::enums::Statements;
use crate::core::traits::statement_trait::Statement;

pub struct ParsedStatement {
    pub statement_type: Statements,

    pub statement: Option<Box<dyn Statement>>,
    pub meta_command: Option<Box<String>>,
}
