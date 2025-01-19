use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::Mutex;

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

    pub async fn load(&self) -> Result<(), DbLoaderLoadError> {
        let db_struct = self.loader.load_structure()?;
        println!("Db parsed struct: {:?}", &db_struct);

        let table_names = db_struct.get_table_names();

        let results = Mutex::new(HashMap::new());

        let table_read_tasks = table_names
            .iter()
            .map(|name| {
                let name = name.to_string();
                let results = &results;

                async move {
                    let table = self.loader.load_table(&name).await;
                    let mut map = results.lock().await;
                    map.insert(name, table);
                };
            })
            .collect();

        Ok(())
    }
}
