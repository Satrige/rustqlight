use crate::core::{
    traits::statement_trait::Statement,
    enums::StatementType,
    types::statements::InsertStatement,
};

pub fn execute(boxed_statement: &Box<dyn Statement>) {
    match boxed_statement.get_statement_type() {
        StatementType::StatementInsert => {
            println!("Statement Insert was not implemented yet");
        },
        StatementType::StatementSelect => {
            println!("Statement Select was not implemented yet");
        },
    }
}
