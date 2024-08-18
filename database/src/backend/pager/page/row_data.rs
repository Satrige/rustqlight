use std::io;

pub const COLUMN_ID_SIZE: usize = size_of::<u32>();
pub const COLUMN_EMAIL_SIZE: usize = 256;
pub const COLUMN_USERNAME_SIZE: usize = 32;

pub struct RowData {
    pub id: u32,
    pub email: String,
    pub user_name: String,
}

impl RowData {
    pub fn new(
        id: u32,
        email: &String,
        user_name: &String,
    ) -> io::Result<RowData> {
        if email.len() > COLUMN_EMAIL_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "The user's mail is too long",
            ));
        }

        if user_name.len() > COLUMN_USERNAME_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "The username is too long",
            ));
        }

        Ok(
            RowData {
                id,
                email: email.clone(),
                user_name: user_name.clone(),
            }
        )
    }

    pub fn print(&self) {
        println!("Id: {}, email: {}, user name: {}", self.id, self.email, self.user_name);
    }
}
