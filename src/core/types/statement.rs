use super::statement_types::{
    InsertStatement,
    SelectStatement,
};

pub enum Statement {
    Insert(InsertStatement),
    Select(SelectStatement),
}