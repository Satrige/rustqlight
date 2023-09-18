use super::prompt;
use super::parser::Parser;

pub fn run() {
    loop {
        prompt::print_prompt();
        let input_buffer = prompt::read_input();
        Parser::parse(&input_buffer);
    }
}
