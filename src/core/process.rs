use super::prompt;
use super::parser;

pub fn run() {
    loop {
        prompt::print_prompt();
        let input_buffer = prompt::read_input();
        parser::parse(&input_buffer);
    }
}
