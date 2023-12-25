use crate::core::database::types::row::{
    Row,
    COLUMN_EMAIL_SIZE,
    COLUMN_USERNAME_SIZE,
};

impl Row {
    pub fn new(
        id: u32,
        email: &String,
        user_name: &String,
    ) -> Result<Row, &'static str> {
        if email.len() > COLUMN_EMAIL_SIZE {
            return Err("The user's mail is too long");
        }

        if user_name.len() > COLUMN_USERNAME_SIZE {
            return Err("The username is too long");
        }

        Ok(
            Row {
                id: id,
                email: email.clone(),
                user_name: user_name.clone(),
            }
        )
    }
}