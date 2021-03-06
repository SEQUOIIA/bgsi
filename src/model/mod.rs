use std::sync::Arc;
use std::error::Error;

pub type STResult<T> = Result<T, STError>;
pub type SendSync<T> = Arc<crossbeam::sync::ShardedLock<T>>;

pub mod github_push;

#[derive(Debug)]
pub enum STError {
    Unknown(Box<dyn Error + Send>),
    NoSuchProvider,
    NoHandler,
    SupplierNotFound,
    ReceiverNotFound,
    HandlerNoResponse,
    GitRefNotFound,
    HandlerErr(String),
    Error(String),

    // Other libraries
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error)
}

impl From<std::io::Error> for STError {
    fn from(e: std::io::Error) -> Self {
        STError::IoError(e)
    }
}

impl From<serde_json::Error> for STError {
    fn from(e: serde_json::Error) -> Self {
        STError::JsonError(e)
    }
}

impl From<serde_yaml::Error> for STError {
    fn from(e: serde_yaml::Error) -> Self {
        STError::YamlError(e)
    }
}

impl From<&str> for STError {
    fn from(e: &str) -> Self {
        STError::Error(e.to_owned())
    }
}