#[derive(Debug)]
pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub enum Statements {
    Statement,
    MetaCommand,
}
