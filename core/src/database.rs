use std::collections::HashMap;
use thiserror::Error;

use crate::{db_config::DbConfig, db_loader::DbLoader, table::Table};

pub struct Database {
    loader: Box<dyn DbLoader>,
    tables: HashMap<String, Table>,
}

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
