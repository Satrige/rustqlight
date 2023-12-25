use crate::core::{
    types::process::Process,
    database::types::table::Table,
    parser::commandParser::parse_with_prompt,
};

impl Process {
    pub fn new() -> Self {
        Process {
            table: Box::new(Table::new()),
        }
    }

    pub fn run(&mut self) {
        loop {
            match parse_with_prompt() {
                Some(parsed_statement) => {
                    self.table.execute(&parsed_statement);
                },
                None => {
                    println!("Unrecognized statement");
                }
            }
        }
    }

}