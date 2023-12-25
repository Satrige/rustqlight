use crate::core::database::types::{
    row::{
        Row,
        COLUMN_EMAIL_SIZE,
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

    fn get_email_bytes(email: &String) {
        let mut email_padded_bytes: Vec<u8> = Vec::with_capacity(COLUMN_EMAIL_SIZE);
    }

    pub fn insert_row(&mut self, row: &Row) {
        let id_bytes = row.id.to_be_bytes(); // 32
        let email_bytes = row.email.as_bytes();
        let user_name_bytes = row.user_name.as_bytes();
    }
}