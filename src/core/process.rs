use super::prompt;
use super::parser;

pub fn run() {
    loop {
        prompt::print_prompt();
        let input_buffer = prompt::read_input();

        match parser::parse(&input_buffer) {
            Some(boxed_statement) => {
                println!("Right way: {:?}", (*boxed_statement).get_statement_type());
            },
            None => println!("Do nothing"),
        }
    }
}
