use crate::core::{
    traits::statement_trait::Statement,
    types::statements::{InsertStatement, SelectStatement},
    enums::StatementType,
};

impl Statement for InsertStatement {
    fn get_statement_type(&self) -> StatementType {
        return StatementType::StatementInsert;
    }
}

impl Statement for SelectStatement {
    fn get_statement_type(&self) -> StatementType {
        return StatementType::StatementSelect;
    }
}
