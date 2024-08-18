pub mod insert_statement;
pub mod select_statement;

use insert_statement::InsertStatement;
use select_statement::SelectStatement;

pub enum Statement {
    Insert(InsertStatement),
    Select(SelectStatement),
}
