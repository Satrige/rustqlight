use crate::core::database::types::table::Table;

impl Table {
    pub fn new() -> Self {
        Table {
            num_rows: 0,
            pages: Vec::new(),
        }
    }
}