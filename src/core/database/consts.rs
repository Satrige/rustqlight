pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

pub const TABLE_MAX_PAGES: usize = 100;

pub const PAGE_SIZE: usize = 4096;

pub const ROWS_PER_PAGE:usize = PAGE_SIZE / ROW_SIZE;