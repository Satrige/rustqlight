mod indexer;

use std::{io, process};
use super::pager::page::{
    PAGE_SIZE,
    row_data::RowData,
};
use indexer::Indexer;
use crate::backend::pager::Pager;
use crate::statement::Statement;
use crate::enums::Statements;
use crate::parser::ParsedStatement;

pub const TABLE_MAX_PAGES: usize = 100;

pub struct Table {
    pager: Pager,
    indexer: Indexer,
    max_num_rows: usize,
}

impl Table {
    pub fn new(pager: Pager) -> Self {
        let last_saved_index = pager.get_last_saved_index().unwrap_or(0);
        Table {
            pager,
            indexer: Indexer::new(Some(last_saved_index)),
            max_num_rows: PAGE_SIZE * TABLE_MAX_PAGES,
        }
    }

    fn insert_new_row(&mut self, new_row_data: &RowData) -> io::Result<usize> {
        self.pager.insert_new_row(new_row_data)
    }

    fn select(&self) -> io::Result<usize> {
        self.pager.select_all();
        Ok(0)
    }

    fn execute_statement(&mut self, statement: &Statement) -> io::Result<usize>{
        match statement {
            Statement::Insert(insert_statement) => {
                let new_row = RowData::new(
                    self.indexer.get_next_index(),
                    &insert_statement.email,
                    &insert_statement.user_name,
                )?;

                self.insert_new_row(&new_row)
            },
            Statement::Select(_select_statement) => {
                self.select()
            },
        }
    }

    fn execute_meta_command(
        &mut self,
        parsed_meta_command: &Option<String>,
    ) -> io::Result<usize> {
        let meta_command = parsed_meta_command.as_ref().unwrap();

        match meta_command.as_str() {
            ".exit" => {
                if let Err(err) = self.pager.save() {
                    println!("Can't dump table to the file: {:?}", err);
                }

                process::exit(0);
            },
            _ => {
                println!("The meta command was not implemented yet");
                Ok(0)
            }
        }
    }

    pub fn execute(&mut self, parsed_statement: ParsedStatement) -> io::Result<usize> {
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
