use std::env;
use std::error::Error;

#[derive(Debug)]
pub enum SfdxError {
    VarError(env::VarError),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl Error for SfdxError {}

impl std::fmt::Display for SfdxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SfdxError::VarError(e) => write!(f, "Environment variable error: {}", e),
            SfdxError::IoError(e) => write!(f, "I/O error: {}", e),
            SfdxError::JsonError(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl From<env::VarError> for SfdxError {
    fn from(err: env::VarError) -> Self {
        SfdxError::VarError(err)
    }
}

impl From<std::io::Error> for SfdxError {
    fn from(err: std::io::Error) -> Self {
        SfdxError::IoError(err)
    }
}

impl From<serde_json::Error> for SfdxError {
    fn from(err: serde_json::Error) -> Self {
        SfdxError::JsonError(err)
    }
}
