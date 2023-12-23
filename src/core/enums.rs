#[derive(Debug)]
pub enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementNull,
}

pub enum Statements {
    Statement,
    MetaCommand,
}
