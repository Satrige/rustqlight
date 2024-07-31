use crate::core::types::statement_types::InsertStatement;

impl InsertStatement {
    pub fn new(words: &Vec<&str>) -> Self {
        if words.len() < 2 {
            panic!("Wrong number of arguments for Insert Statement");
        }

        InsertStatement {
            email: words[1].to_string(),
            user_name: words[2].to_string(),
        }
    }
}