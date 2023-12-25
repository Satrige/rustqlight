use crate::core::database::types::page::Page;

impl Page {
    pub fn new() -> Self {
        Page {
            num_rows: 0,
            destination: Vec::new(),
        }
    }
}