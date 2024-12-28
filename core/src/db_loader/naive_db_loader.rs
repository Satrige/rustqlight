use super::{DbLoader, DbLoaderError};
use log::error;
use std::fs;

/// The loader which loads the whole db from a file
struct NaiveDbLoader {}

impl DbLoader for NaiveDbLoader {
    fn load(file_name: &str) -> Result<(), DbLoaderError> {
        match fs::read_to_string(file_name) {
            Ok(file_contents) => Ok(()),
            Err(err) => {
                error!(
                    "Can't read the db file: {}.\nDescription: {}.",
                    file_name, err,
                );
                Err(DbLoaderError::WrongDbPath(file_name.to_string()))
            }
        }
    }
}
