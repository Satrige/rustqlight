pub struct Indexer {
    cur_index: u32,
}

impl Indexer {
    pub fn new(last_idx: Option<u32>) -> Self {
        Indexer {
            cur_index: match last_idx {
                Some(last_idx_val) => last_idx_val + 1,
                None => 1,
            }
        }
    }

    pub fn get_next_index(&mut self) -> u32 {
        let result = self.cur_index;
        self.cur_index += 1;

        result
    }
}