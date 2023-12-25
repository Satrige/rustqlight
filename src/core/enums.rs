#[derive(Debug)]
pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

#[derive(PartialEq)]
pub enum Statements {
    Statement,
    MetaCommand,
}
