mod naive_db_loader;
use thiserror::Error;

pub use naive_db_loader::NaiveDbLoader;

#[derive(Error, Debug)]
pub enum DbLoaderLoadError {
    #[error("Wrong config path: {0}")]
    WrongDbPath(String),
}

#[derive(Error, Debug)]
pub enum DbLoaderDumpError {
    #[error("Wrong config path: {0}")]
    WrongDbPath(String),
}

pub trait DbLoader {
    fn load_structure(&self, dir_name: &str) -> Result<(), DbLoaderLoadError>;

    fn dump(&self) -> Result<(), DbLoaderDumpError>;
}
