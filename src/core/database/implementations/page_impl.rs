use crate::core::database::types::{
    row::Row,
    page::Page,
};

impl Page {
    pub fn new() -> Self {
        Page {
            num_rows: 0,
            destination: Vec::new(),
        }
    }

    pub fn insert_row(row: &Row) {
        println!("Not implemented yet");
    }
}