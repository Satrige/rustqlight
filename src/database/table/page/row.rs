pub const COLUMN_EMAIL_SIZE: usize = 256;
pub const COLUMN_USERNAME_SIZE: usize = 32;

pub struct Row {
    pub id: u32,
    pub email: String,
    pub user_name: String,
}

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
                id,
                email: email.clone(),
                user_name: user_name.clone(),
            }
        )
    }
}
