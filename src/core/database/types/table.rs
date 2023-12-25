use super::page::Page;

pub const TABLE_MAX_PAGES: usize = 100;

pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
    pub max_num_rows: usize,
}
