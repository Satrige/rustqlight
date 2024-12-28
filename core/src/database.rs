use thiserror::Error;

use crate::db_config::DbConfig;

pub struct Database {}

#[derive(Error, Debug)]
pub enum DatabaseCreationError {
    #[error("Wrong config path: `{0}`")]
    WrongConfigPath(String),
}

impl Database {
    pub fn new(config: &DbConfig) -> Result<Self, DatabaseCreationError> {
        Ok(Database {})
    }
}
