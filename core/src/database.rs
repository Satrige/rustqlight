use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::{sync::Mutex, task};

use crate::{
    db_config::DbConfig,
    db_loader::{DbLoader, DbLoaderLoadError, NaiveDbLoader},
    table::Table,
};

pub struct Database {
    loader: Arc<dyn DbLoader>,
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
            loader: Arc::new(NaiveDbLoader::new(config.db_path.clone())),
            tables: HashMap::new(),
        })
    }

    pub async fn load(&self) -> Result<(), DbLoaderLoadError> {
        let db_struct = self.loader.load_structure()?;
        println!("Db parsed struct: {:?}", &db_struct);

        let table_names = db_struct.get_table_names();

        let results = Mutex::new(HashMap::new());

        let mut handlers = Vec::new();

        for name in table_names {
            let name = name.to_string();
            let results = &results;

            let clonned_loader = Arc::clone(&self.loader);

            let handle = task::spawn(async move {
                let table = clonned_loader.load_table(&name).await;
                let mut map = results.lock().await;
                map.insert(name, table);
            });

            handlers.push(handle);
        }

        for handle in handlers {
            handle.await.unwrap();
        }

        let results = results.lock().await;
        for (key, table) in results.iter() {
            println!("Loaded table: {} -> {:?}", key, table.get_name());
        }

        Ok(())
    }
}
