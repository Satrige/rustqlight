use std::io::{ self, Write };
use crate::core::types::input_buffer::InputBuffer;

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn read_input() -> InputBuffer {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    let mut input_buffer = InputBuffer::new();

    input_buffer.input_length = input.len();
    input_buffer.buffer = Some(Box::new(input));

    return input_buffer;
}

pub fn get_input() -> InputBuffer {
    print_prompt();
    return read_input();
}
