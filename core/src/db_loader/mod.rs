mod naive_db_loader;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbLoaderError {
    #[error("Wrong config path: {0}")]
    WrongDbPath(String),
}

pub trait DbLoader {
    fn load(file_name: &str) -> Result<(), DbLoaderError>;
}
