mod input_buffer;
mod promt;

use crate::input_buffer::InputBuffer;
use crate::promt::{print_promt, read_input};

fn main() {
    let input_buffer = &mut InputBuffer::new();

    loop {
        print_promt();
        read_input(input_buffer);

        println!("{:?}", input_buffer);
    }
}
