pub const PAGE_SIZE: usize = 4096;

pub struct Page {
    pub num_rows: usize,
    pub destination: Vec<u8>,
}