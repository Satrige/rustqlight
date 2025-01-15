use std::collections::HashMap;
use thiserror::Error;

use crate::{
    db_config::DbConfig,
    db_loader::{DbLoader, DbLoaderLoadError, NaiveDbLoader},
    table::Table,
};

pub struct Database {
    loader: Box<dyn DbLoader>,
    tables: HashMap<String, Table>,
}

#[derive(Error, Debug)]
pub enum DatabaseCreationError {
    #[error("Wrong config path: `{0}`")]
    WrongConfigPath(String),

    #[error("Wrong loader type: `{0}`")]
    WrongLoaderType(String),
}

impl Database {
    pub fn new(config: &DbConfig) -> Result<Self, DatabaseCreationError> {
        if config.loader.as_str() != "naive" {
            return Err(DatabaseCreationError::WrongLoaderType(
                config.loader.clone(),
            ));
        }

        Ok(Database {
            loader: Box::new(NaiveDbLoader::new(config.db_path.clone())),
            tables: HashMap::new(),
        })
    }

    pub fn load(&self) -> Result<(), DbLoaderLoadError> {
        let db_struct = self.loader.load_structure()?;

        println!("Db parsed struct: {:?}", &db_struct);

        Ok(())
    }
}
