use log::error;
use serde::Deserialize;
use serde_yaml;
use std::path::{Path, PathBuf};
use std::{fs, io};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub db_path: String,
    pub loader: String,
}

#[derive(Error, Debug)]
pub enum DbConfigError {
    #[error("Can't canonicalize the config path: {0}")]
    CanonError(#[from] io::Error),

    #[error("The config file doesn't exist: {0}")]
    NotExistConfigFile(PathBuf),

    #[error("Wrong config path: {0}")]
    WrongConfigPath(PathBuf),

    #[error("Wrong format of the config: {0}")]
    WrongConfigFormat(String),
}

impl DbConfig {
    pub fn new() -> Self {
        DbConfig {
            db_path: "./db".to_string(),
            loader: "naive".to_string(),
        }
    }

    pub fn parse_config_file<P: AsRef<Path>>(
        config_file_path: P,
    ) -> Result<DbConfig, DbConfigError> {
        let canon_file_path = fs::canonicalize(&config_file_path)?;

        if !canon_file_path.exists() {
            return Err(DbConfigError::NotExistConfigFile(canon_file_path));
        }

        match fs::read_to_string(file_name) {
            Ok(file_contents) => match serde_yaml::from_str(&file_contents) {
                Ok(db_config) => Ok(db_config),
                Err(err) => {
                    error!(
                        "Can't deserialize the conf file: {}.\nDescription: {}.",
                        file_name, err,
                    );
                    Err(DbConfigError::WrongConfigFormat(file_name.to_string()))
                }
            },
            Err(err) => {
                error!(
                    "Can't read the conf file: {}.\nDescription: {}.",
                    file_name, err,
                );
                Err(DbConfigError::WrongConfigPath(file_name.to_string()))
            }
        }
    }
}
