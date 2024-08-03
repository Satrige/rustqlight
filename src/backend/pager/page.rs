pub mod row_data;
mod deserializer;

use std::io;
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
    num_rows: usize,
    destination: Vec<u8>,
    last_idx: u32,
}

impl Page {
    pub fn new(data: Option<&[u8]>) -> Self {
        match data {
            Some(rows) => {
                let (last_idx, num_rows, destination_rows) = Self::parse_raw_data(&rows);

                Page {
                    num_rows,
                    last_idx,
                    destination: destination_rows.to_vec(),
                }
            },
            None => Page {
                num_rows: 0,
                destination: Vec::new(),
                last_idx: 0,
            }
        }
    }

    /// The method parses raw bytes data
    fn parse_raw_data(data: &[u8]) -> (u32, usize, &[u8]) {
        // Get the index of the last non-nullable row
        let last_non_empty_row_idx = (0..PAGE_SIZE / ROW_SIZE)
            .rev()
            .find(|&i| {
                let start = i * ROW_SIZE;
                let end = start + ROW_SIZE;
                data[start..end].iter().any(|&byte| byte != 0)
            })
            .map(|i| i + 1)
            .unwrap_or(0);

        (
            U32Deserializer::deserialize(
                &data[(last_non_empty_row_idx - 1) * ROW_SIZE..last_non_empty_row_idx * ROW_SIZE],
                0,
                COLUMN_ID_SIZE,
            ),
            last_non_empty_row_idx,
            &data[..last_non_empty_row_idx * ROW_SIZE],
        )
    }

    fn get_value_padded(&self, value_str: &String, length: usize) -> Vec<u8> {
        let value_bytes = value_str.as_bytes();

        // Create a Vec with predefined capacity without allocation
        let mut result: Vec<u8> = Vec::with_capacity(length);
        // file by bytes from slice
        result.extend_from_slice(value_bytes);
        // Fill by zeroes
        result.resize(length, 0);

        result
    }

    // TODO Need to rewrite it by preparing the data and then copying it to the destination
    pub fn insert_row(&mut self, row_data: &RowData) -> io::Result<usize> {
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
        self.last_idx = row_data.id;

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

    pub fn get_last_idx(&self) -> u32 {
        self.last_idx
    }

    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn get_destination(&self) -> &[u8] {
        &self.destination
    }
}
