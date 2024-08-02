mod page;

use std::process;
use page::{
    Page,
    PAGE_SIZE,
    row_data::RowData,
};
use crate::statement::Statement;
use crate::enums::Statements;
use crate::parser::ParsedStatement;

const TABLE_MAX_PAGES: usize = 100;

pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
    pub max_num_rows: usize,
}

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

    fn insert_new_row(&mut self, new_row_data: &RowData) -> Result<usize, &'static str> {
        // Update it in advance
        if self.num_rows % PAGE_SIZE == 0 {
            self.pages.push(Page::new());
        }

        let last_page: &mut Page = self.pages.last_mut().unwrap();

        last_page.insert_row(new_row_data).unwrap();

        self.num_rows += 1;

        Ok(0)
    }

    fn select(&self) -> Result<usize, &'static str> {
        for page in self.pages.iter() {
            page.print();
        }

        Ok(0)
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<usize, &'static str>{
        match statement {
            Statement::Insert(insert_statement) => {
                if let Ok(new_id) = self.get_new_id() {
                    if let Ok(new_row) = RowData::new(
                        new_id.try_into().unwrap(),
                        &insert_statement.email,
                        &insert_statement.user_name,
                    ) {
                        self.insert_new_row(&new_row)
                    } else {
                        Err("Can't prepare new row data")
                    }
                } else {
                    Err("Can't obtain the new row id")
                }
            },
            Statement::Select(_select_statement) => {
                self.select()
            },
        }
    }

    fn execute_meta_command(
        &self,
        parsed_meta_command: &Option<String>,
    ) -> Result<usize, &'static str> {
        let meta_command = parsed_meta_command.as_ref().unwrap();

        match meta_command.as_str() {
            ".exit" => {
                process::exit(0);
            },
            _ => {
                println!("The metacommand was not implemented yet");
                Ok(0)
            }
        }
    }

    pub fn execute(&mut self, parsed_statement: ParsedStatement) ->Result<usize, &'static str> {
        match parsed_statement.statement_type {
            Statements::Statement => {
                self.execute_statement(parsed_statement.statement.as_ref().unwrap())
            },
            Statements::MetaCommand => {
                self.execute_meta_command(&parsed_statement.meta_command)
            },
        }
    }
}
