use crate::parser;
use crate::database;

pub struct Process {
    table: Box<database::table::Table>,
}

impl Process {
    pub fn new() -> Self {
        Process {
            table: Box::new(database::table::Table::new()),
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
