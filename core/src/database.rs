use thiserror::Error;

pub struct Database {}

#[derive(Error, Debug)]
pub enum DatabaseCreationError {
    #[error("Wrong config path: `{0}`")]
    WrongConfigPath(String),
}

impl Database {
    pub fn new() -> Result<Self, DatabaseCreationError> {

    }
}