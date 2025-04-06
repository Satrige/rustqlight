use crate::column_type::ColumnType;
use std::iter::Map;

pub struct Table {
    name: String,
    schema: Map<String, ColumnType>,
}

impl Table {
    pub fn new(table_name: &str, schema: Map<String, ColumnType>) -> Self {
        Self {
            name: table_name.to_string(),
            schema,
        }
    }

    pub fn add_row() {}

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
