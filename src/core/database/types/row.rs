pub const COLUMN_EMAIL_SIZE: usize = 256;
pub const COLUMN_USERNAME_SIZE: usize = 32;

pub struct Row {
    pub id: u32,
    pub email: String,
    pub user_name: String,
}
