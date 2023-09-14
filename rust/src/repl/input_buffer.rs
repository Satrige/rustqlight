#[derive(Debug)]
pub struct InputBuffer {
    pub input_length: usize,
    pub buffer: Option<Box<String>>,
}

impl InputBuffer {
    pub fn new() -> Self {
        InputBuffer {
            input_length: 0,
            buffer: None,
        }
    }
}