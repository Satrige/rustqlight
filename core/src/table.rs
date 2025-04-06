use crate::row::Row;

pub struct Table {
    name: String,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(table_name: &str) -> Self {
        Self {
            name: table_name.to_string(),
            rows: Vec::new(),
        }
    }

    pub fn add_row() {}

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
