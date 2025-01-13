use super::{DbLoader, DbLoaderDumpError, DbLoaderLoadError};
use log::error;
use serde::Deserialize;
use serde_yaml;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct RowStruct {
    row_name: String,
    row_type: String,
}

#[derive(Debug, Deserialize)]
struct TableStruct {
    table_name: String,
    structure: Vec<RowStruct>,
}

#[derive(Debug, Deserialize)]
pub struct DbStruct {
    db_name: String,
    tables: Vec<TableStruct>,
}

/// The loader which loads the whole db from a file
pub struct NaiveDbLoader {}

impl NaiveDbLoader {
    const DEFAULT_DB_STRUCT_FILE_NAME: &'static str = "db_struct.yaml";
    pub fn new() -> Self {
        Self {}
    }
}

impl DbLoader for NaiveDbLoader {
    fn load_structure(&self, dir_name: &str) -> Result<(), DbLoaderLoadError> {
        let db_struct_path = Path::new(dir_name).join(NaiveDbLoader::DEFAULT_DB_STRUCT_FILE_NAME);

        match fs::read_to_string(&db_struct_path) {
            Ok(file_contents) => {
                let db_struct: DbStruct = serde_yaml::from_str(&file_contents)?;
                Ok(())
            }
            Err(err) => {
                error!(
                    "Can't read the db file: {:?}.\nDescription: {}.",
                    db_struct_path, err,
                );
                Err(DbLoaderLoadError::WrongDbPath(
                    db_struct_path.into_os_string().into_string().unwrap(),
                ))
            }
        }
    }

    fn dump(&self) -> Result<(), DbLoaderDumpError> {
        Ok(())
    }
}
