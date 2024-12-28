use log::error;
use serde::Deserialize;
use serde_yaml;
use std::fs;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    db_path: String,
}

#[derive(Error, Debug)]
pub enum DbConfigError {
    #[error("Wrong config path: {0}")]
    WrongConfigPath(String),

    #[error("Wrong format of the config: {0}")]
    WrongConfigFormat(String),
}

impl DbConfig {
    pub fn new() -> Self {
        DbConfig {
            db_path: "./db_config.yaml".to_string(),
        }
    }

    pub fn parse_config_file(file_name: &str) -> Result<DbConfig, DbConfigError> {
        match fs::read_to_string(file_name) {
            Ok(file_contents) => match serde_yaml::from_str(&file_contents) {
                Ok(db_config) => Ok(db_config),
                Err(err) => {
                    error!(
                        "Can't deserialize the conf file: {}.\nDescription: {}",
                        file_name, err,
                    );
                    Err(DbConfigError::WrongConfigFormat(file_name.to_string()))
                }
            },
            Err(err) => {
                error!(
                    "Can't read the conf file: {}.\nDescription: {}",
                    file_name, err,
                );
                Err(DbConfigError::WrongConfigPath(file_name.to_string()))
            }
        }
    }
}
