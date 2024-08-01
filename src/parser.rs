use std::io::{ self, Write };

use crate::statement::{
    Statement,
    insert_statement::InsertStatement,
    select_statement::SelectStatement,
};
use crate::enums::Statements;

pub struct ParsedStatement {
    pub statement_type: Statements,

    pub statement: Option<Statement>,
    pub meta_command: Option<String>,
}

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

struct InputBuffer {
    pub input_length: usize,
    pub buffer: Option<Box<String>>,
}

fn print_prompt() {
    print!("database > ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn read_input() -> InputBuffer {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    InputBuffer {
        input_length: input.len(),
        buffer: Some(Box::new(input)),
    }
}

fn get_input() -> InputBuffer {
    print_prompt();

    read_input()
}

pub fn parse_with_prompt() -> Option<ParsedStatement> {
    let input_buffer = get_input();

    return match &input_buffer.buffer {
        Some(buffer) => {
            let parsed_statement = ParsedStatement::new(buffer);

            if parsed_statement.statement_type == Statements::Statement {
                match &parsed_statement.statement {
                    Some(_) => {
                        Some(parsed_statement)
                    },
                    None => {
                        None
                    },
                }
            } else {
                Some(parsed_statement)
            }
        },
        None => {
            println!("Wrong way");

            None
        }
    };
}

