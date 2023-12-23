use super::parser::commandParser::parse_with_prompt;

pub fn initialize() {
    println!("The db was initialized");
}

pub fn run() {
    loop {
        parse_with_prompt();

        // match parser::parse(&input_buffer) {
        //     Some(boxed_statement) => {
        //         println!("Right way: {:?}", (*boxed_statement).get_statement_type());
        //     },
        //     None => println!("Do nothing"),
        // }
    }
}
