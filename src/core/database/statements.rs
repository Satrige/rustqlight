use crate::core::{
    types::{
        statement::Statement,
    },
};

pub fn execute(boxed_statement: &Box<Statement>) {
    match &**boxed_statement {
        Statement::Insert(insert_statement) => {
            println!("Statement Insert was not implemented yet");
        },
        Statement::Select(select_statement) => {
            println!("Statement Select was not implemented yet");
        },
    }
}
