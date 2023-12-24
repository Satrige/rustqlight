use super::{
    parser::commandParser::parse_with_prompt,
    database::execution,
};

pub fn initialize() {
    println!("The db was initialized");
}

pub fn run() {
    loop {
        match parse_with_prompt() {
            Some(parsed_statement) => {
                execution::execute(&parsed_statement);
            },
            None => {
                println!("Unrecognized statement");
            }
        }
    }
}
