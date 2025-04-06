use crate::table::Table;

use super::{DbLoader, DbLoaderDumpError, DbLoaderLoadError, DbLoaderLoadTableError, DbStruct};
use async_trait::async_trait;
use log::error;
use std::fs;
use std::path::Path;

/// The loader which loads the whole db from a file
pub struct NaiveDbLoader {
    db_path: String,
}

impl NaiveDbLoader {
    const DEFAULT_DB_STRUCT_FILE_NAME: &'static str = "db_struct.yaml";
    pub fn new(db_path: String) -> Self {
        Self { db_path }
    }
}

impl NaiveDbLoader {
    fn parse_struct_file(&self, file_contents: &str) -> Result<DbStruct, DbLoaderLoadError> {
        match serde_yaml::from_str(file_contents) {
            Ok(db_struct) => Ok(db_struct),
            Err(err) => {
                error!(
                    "Can't deserialize the struct data: {}.\nDescription: {}.",
                    file_contents, err,
                );
                Err(DbLoaderLoadError::MalformedData)
            }
        }
    }
}

#[async_trait]
impl DbLoader for NaiveDbLoader {
    fn load_structure(&self) -> Result<DbStruct, DbLoaderLoadError> {
        let db_struct_path =
            Path::new(&self.db_path).join(NaiveDbLoader::DEFAULT_DB_STRUCT_FILE_NAME);

        match fs::read_to_string(&db_struct_path) {
            Ok(file_contents) => self.parse_struct_file(&file_contents),
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

    async fn load_table(&self, table_name: &str) -> Result<Table, DbLoaderLoadTableError> {
        let table_data_path = Path::new(&self.db_path).join(table_name);

        let canon_table_data_path = fs::canonicalize(&table_data_path)?;

        if !canon_table_data_path.exists() {
            return Err(DbLoaderLoadTableError::WrongTableName(
                table_name.to_string(),
            ));
        }

        match fs::read_to_string(&table_data_path) {
            // TODO Write the logic here
            Ok(_table_contents) => {
                let table = Table::new(table_name);
                Ok(table)
            }
            Err(err) => {
                error!(
                    "Can't read the table file: {:?}.\nDescription: {}.",
                    table_data_path, err,
                );
                Err(DbLoaderLoadTableError::ReadTableError(
                    table_name.to_string(),
                    table_data_path,
                ))
            }
        }
    }

    fn dump(&self) -> Result<(), DbLoaderDumpError> {
        Ok(())
    }
}
