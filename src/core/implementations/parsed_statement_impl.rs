use crate::core::{
    enums::Statements,
    types::{
        parsed_statement::ParsedStatement,
        statement_types::{
            InsertStatement,
            SelectStatement,
        },
        statement::Statement,
    },
};

impl ParsedStatement {
    fn prepare_statement(statement_str: &String) -> Option<Statement> {
        let words: Vec<&str> = statement_str.split_whitespace().collect();
        let words_count = words.len();

        if words_count >= 3 && words[0].to_string() == "insert" {
            let insert_statement = InsertStatement::new(&words);
            return Some(Statement::Insert(insert_statement));
        }

        if words_count >= 3 && words[0].to_string() == "select" {
            let select_statement = SelectStatement::new(&words);
            return Some(Statement::Select(select_statement));
        }

        return None;
    }

    pub fn new(buffer: &Box<String>) -> Self {
        if buffer.chars().next() == Some('.') {
            return ParsedStatement {
                statement_type: Statements::MetaCommand,
                statement: None,
                meta_command: Some((**buffer).clone()),
            }
        } else {
            return ParsedStatement {
                statement_type: Statements::Statement,
                statement: ParsedStatement::prepare_statement(buffer),
                meta_command: None,
            }
        }
    } 
}
