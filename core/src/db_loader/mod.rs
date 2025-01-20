mod naive_db_loader;
use std::{io, path::PathBuf};

use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

pub use naive_db_loader::NaiveDbLoader;

use crate::table::Table;

#[derive(Error, Debug)]
pub enum DbLoaderLoadError {
    #[error("Wrong config path: {0}")]
    WrongDbPath(String),

    #[error("Wrong format of the database data")]
    MalformedData,
}

#[derive(Error, Debug)]
pub enum DbLoaderLoadTableError {
    #[error("Wrong table name: {0}")]
    WrongTableName(String),

    #[error("Can't read table {0} from file: {1}")]
    ReadTableError(String, PathBuf),

    #[error("Can't canonicalize the table path: {0}")]
    CanonError(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum DbLoaderDumpError {
    #[error("Wrong config path: {0}")]
    WrongDbPath(String),
}

#[derive(Debug, Deserialize)]
struct RowStruct {
    row_name: String,
    row_type: String,
}

#[derive(Debug, Deserialize)]
struct TableStruct {
    table_name: String,
    table_structure: Vec<RowStruct>,
}

#[derive(Debug, Deserialize)]
pub struct DbStruct {
    db_name: String,
    tables: Vec<TableStruct>,
}

impl DbStruct {
    pub fn get_table_names(&self) -> Vec<&str> {
        self.tables
            .iter()
            .map(|table| table.table_name.as_str())
            .collect()
    }
}

#[async_trait]
pub trait DbLoader {
    fn load_structure(&self) -> Result<DbStruct, DbLoaderLoadError>;

    async fn load_table(&self, table_name: &str) -> Result<Table, DbLoaderLoadTableError>;

    fn dump(&self) -> Result<(), DbLoaderDumpError>;
}
