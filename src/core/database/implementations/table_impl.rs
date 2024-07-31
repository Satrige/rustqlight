use std::process;

use crate::core::{
    enums::Statements,
    database::types::{
        row::Row,
        page::{
            Page,
            PAGE_SIZE,
        },
        table::{
            Table,
            TABLE_MAX_PAGES,
        },
    },
    types::{
        statement::Statement,
        parsed_statement::ParsedStatement,
    },
};

impl Table {
    pub fn new() -> Self {
        Table {
            num_rows: 0,
            pages: Vec::new(),
            max_num_rows: PAGE_SIZE * TABLE_MAX_PAGES,
        }
    }

    fn get_new_id(&self) -> Result<usize, &'static str> {
        let result = self.num_rows + 1;
        if self.num_rows + 1 < self.max_num_rows {
            Ok(result)
        } else {
            Err("The maximum number of rows exceeded")
        }
    }

    fn insert_new_row(&mut self, new_row: &Row) {
        if self.num_rows % PAGE_SIZE == 0 {
            self.pages.push(Page::new());
        }

        let last_page: &mut Page = self.pages.last_mut().unwrap();
        
        last_page.insert_row(&new_row);
    }

    fn execute_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Insert(insert_statement) => {
                if let Ok(new_id) = self.get_new_id() {
                    if let Ok(new_row) = Row::new(
                        new_id.try_into().unwrap(),
                        &insert_statement.email,
                        &insert_statement.user_name,
                    ) {
                        self.insert_new_row(&new_row);
                    }
                }
            },
            Statement::Select(select_statement) => {
                println!("Statement Select was not implemented yet");
            },
        }
    }

    fn execute_meta_command(&self, parsed_meta_command: &Option<String>) {
        let meta_command = parsed_meta_command.as_ref().unwrap();

        match (&meta_command).as_str() {
            ".exit" => {
                process::exit(0);
            },
            _ => {
                println!("The metacommand was not implemented yet");
            }
        }
    }

    pub fn execute(&mut self, parsed_statement: ParsedStatement) {
        match parsed_statement.statement_type {
            Statements::Statement => {
                self.execute_statement(parsed_statement.statement.as_ref().unwrap());
            },
            Statements::MetaCommand => {
                self.execute_meta_command(&parsed_statement.meta_command);
            },
        }
    }
}