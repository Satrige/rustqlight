use crate::parser;
use crate::backend::{ db_open, Table };

pub struct Process {
    table: Table,
}

impl Process {
    pub fn new() -> Self {
        Process {
            table: db_open(None), // TODO rewrite, using command line db path
        }
    }

    pub fn run(&mut self) {
        loop {
            match parser::parse_with_prompt() {
                Some(parsed_statement) => {
                    let _ = self.table.execute(parsed_statement);
                },
                None => {
                    println!("Unrecognized statement");
                }
            }
        }
    }

}
