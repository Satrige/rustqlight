use super::input_buffer::InputBuffer;

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}


#[derive(Debug)]
pub enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementNull,
}

pub trait Statement {
    fn get_statement_type(&self) -> StatementType;
}

struct InsertStatement {
    pub email: String,
    pub user_name: String,
}

impl Statement for InsertStatement {
    fn get_statement_type(&self) -> StatementType {
        return StatementType::StatementInsert;
    }
}

impl InsertStatement {
    fn new(words: &Vec<&str>) -> Self {
        InsertStatement {
            email: words[1].to_string(),
            user_name: words[2].to_string(),
        }
    }
}

struct SelectStatement;

impl Statement for SelectStatement {
    fn get_statement_type(&self) -> StatementType {
        return StatementType::StatementSelect;
    }
}

impl SelectStatement {
    fn new(words: &Vec<&str>) -> Self {
        SelectStatement
    }
}

pub fn prepare_statement(
    input_buffer: &InputBuffer,
) -> Option<Box<dyn Statement>> {
    let statement_str = match &input_buffer.buffer {
        Some(statement) => { &**statement },
        None => {
            return None;
        },
    };

    let words: Vec<&str> = statement_str.split_whitespace().collect();
    let words_count = words.len();

    if words_count >= 3 && words[0].to_string() == "insert" {
        let insert_statement = InsertStatement::new(&words);
        return Some(Box::new(insert_statement));
    }

    if words_count >= 3 && words[0].to_string() == "select" {
        let select_statement = SelectStatement::new(&words);
        return Some(Box::new(select_statement));
    }

    return None;
}
