use crate::core::enums::StatementType;

pub trait Statement {
    fn get_statement_type(&self) -> StatementType;
}
