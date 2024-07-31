use crate::core::database::types::{
    row::{
        Row,
        COLUMN_EMAIL_SIZE,
        COLUMN_USERNAME_SIZE,
    },
    page::Page,
};

impl Page {
    pub fn new() -> Self {
        Page {
            num_rows: 0,
            destination: Vec::new(),
        }
    }

    fn get_value_padded(&self, value_str: &String, length: usize) -> Vec<u8> {
        let value_bytes = value_str.as_bytes();
        let mut result: Vec<u8> = Vec::with_capacity(length);

        result.extend_from_slice(value_bytes);
        result.resize(length, 0);

        return result;
    }

    pub fn insert_row(&mut self, row: &Row) -> Result<usize, &'static str> {
        self.destination.extend(
            row.id.to_be_bytes(),
        );
        self.destination.extend(
            self.get_value_padded(&row.email, COLUMN_EMAIL_SIZE),
        );
        self.destination.extend(
            self.get_value_padded(&row.user_name, COLUMN_USERNAME_SIZE),
        );

        self.num_rows += 1;

        Ok(self.num_rows)
    }
}