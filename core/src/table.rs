pub struct Table {
    name: String,
}

impl Table {
    pub fn new(table_name: &str) -> Self {
        Self {
            name: table_name.to_string(),
        }
    }
}
