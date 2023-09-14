mod prompt;
mod parser;
mod input_buffer;

use parser::Parser;

pub fn run() {
    loop {
        prompt::print_prompt();
        let input_buffer = prompt::read_input();
        Parser::parse(&input_buffer);
    }
}
