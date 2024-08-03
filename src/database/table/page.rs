pub mod row_data;
mod deserializer;

use deserializer::{
    Deserializer,
    U32Deserializer,
    UTF8Deserializer,
};

use row_data::{
    RowData,
    COLUMN_ID_SIZE,
    COLUMN_EMAIL_SIZE,
    COLUMN_USERNAME_SIZE,
};

pub const PAGE_SIZE: usize = 4096;
const ROW_SIZE: usize = COLUMN_ID_SIZE + COLUMN_EMAIL_SIZE + COLUMN_USERNAME_SIZE;
const EMAIL_OFFSET: usize = COLUMN_ID_SIZE;
const USERNAME_OFFSET: usize = EMAIL_OFFSET + COLUMN_EMAIL_SIZE;


pub struct Page {
    pub num_rows: usize,
    pub destination: Vec<u8>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            num_rows: 0,
            destination: Vec::new(),
        }
    }

    fn get_value_padded(&self, value_str: &String, length: usize) -> Vec<u8> {
        let value_bytes = value_str.as_bytes();

        let mut result: Vec<u8> = Vec::with_capacity(length); // Create a Vec with predefined capacity without allocation
        result.extend_from_slice(value_bytes); // file by bytes from slice
        result.resize(length, 0); // Fill by zeroes

        result
    }

    // TODO Need to rewrite it by preparing the data and then copying it to the destination
    pub fn insert_row(&mut self, row_data: &RowData) -> Result<usize, &'static str> {
        self.destination.extend(
            row_data.id.to_be_bytes(),
        );
        self.destination.extend(
            self.get_value_padded(&row_data.email, COLUMN_EMAIL_SIZE),
        );
        self.destination.extend(
            self.get_value_padded(&row_data.user_name, COLUMN_USERNAME_SIZE),
        );

        self.num_rows += 1;

        Ok(self.num_rows)
    }

    fn deserialize_row(&self, row_num: usize) -> RowData {
        let offset = row_num * ROW_SIZE;

        RowData::new(
            U32Deserializer::deserialize(&self.destination, offset, COLUMN_ID_SIZE),
            &UTF8Deserializer::deserialize(
                &self.destination,
                offset + EMAIL_OFFSET,
                COLUMN_EMAIL_SIZE,
            ),
            &UTF8Deserializer::deserialize(
                &self.destination,
                offset + USERNAME_OFFSET,
                COLUMN_USERNAME_SIZE,
            ),
        ).unwrap()
    }

    pub fn print(&self) {
        for i in 0..self.num_rows {
            self.deserialize_row(i).print();
        }
    }
}
