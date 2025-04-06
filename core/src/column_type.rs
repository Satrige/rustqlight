pub enum ColumnType {
    Integer,
    String,
}

impl ColumnType {
    pub fn get_size(&self, column_type: &ColumnType) -> usize {
        match column_type {
            ColumnType::Integer => 4,
            ColumnType::String => 256,
        }
    }
}
