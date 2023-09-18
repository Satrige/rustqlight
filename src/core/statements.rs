use super::input_buffer::InputBuffer;

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementNull,
}

pub struct Statement {
    statement_type: StatementType,
}

impl Statement {
    pub fn new() -> Self {
        Statement {
            statement_type: StatementType::StatementNull,
        }
    }
}

pub fn prepare_statement(
    input_buffer: &InputBuffer,
    statement: &mut Statement,
) -> PrepareResult {
    let statement_str = match &input_buffer.buffer {
        Some(statement) => { (*statement).as_str() },
        None => {
            return PrepareResult::PrepareUnrecognizedStatement;
        }
    };

    if statement_str.starts_with("insert") {
        statement.statement_type = StatementType::StatementInsert;
        return PrepareResult::PrepareSuccess;
    }

    if statement_str.starts_with("select") {
        statement.statement_type = StatementType::StatementSelect;
        return PrepareResult::PrepareSuccess;
    }

    return PrepareResult::PrepareUnrecognizedStatement;
}
