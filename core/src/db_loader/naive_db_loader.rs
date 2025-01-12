use super::{DbLoader, DbLoaderDumpError, DbLoaderLoadError};
use log::error;
use std::fs;

/// The loader which loads the whole db from a file
pub struct NaiveDbLoader {}

impl NaiveDbLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl DbLoader for NaiveDbLoader {
    fn load(&self, file_name: &str) -> Result<(), DbLoaderLoadError> {
        match fs::read_to_string(file_name) {
            Ok(file_contents) => Ok(()),
            Err(err) => {
                error!(
                    "Can't read the db file: {}.\nDescription: {}.",
                    file_name, err,
                );
                Err(DbLoaderLoadError::WrongDbPath(file_name.to_string()))
            }
        }
    }

    fn dump(&self) -> Result<(), DbLoaderDumpError> {
        Ok(())
    }
}
