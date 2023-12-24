use crate::core::types::statements::InsertStatement;

impl InsertStatement {
    pub fn new(words: &Vec<&str>) -> Self {
        InsertStatement {
            email: words[1].to_string(),
            user_name: words[2].to_string(),
        }
    }
}