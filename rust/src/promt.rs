use std::io::{self, Write};
use crate::input_buffer::InputBuffer;

pub fn print_promt() {
    print!("db > ");
    io::stdout().flush().expect("Failed to flush stdout");
}

pub fn read_input(input_buffer: &mut InputBuffer) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    input_buffer.input_length = input.len();
    input_buffer.buffer = Some(Box::new(input));
}