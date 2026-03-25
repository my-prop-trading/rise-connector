use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Rest API Error: {0}")]
    RestError(String),

    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("Parse float error")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("JSON error")]
    Json(#[from] serde_json::Error),

    #[error("Timestamp error")]
    Timestamp(#[from] std::time::SystemTimeError),

    #[error("{0}")]
    Message(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Message(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Message(s.to_string())
    }
}