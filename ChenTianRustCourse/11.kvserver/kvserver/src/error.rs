use std::io::Error;

use thiserror::Error;

use crate::Value;


#[derive(Error, Debug, PartialEq)]
pub enum KvError {
    #[error("Not Found for table: {0}. key: {1}")]
    NotFound(String, String),
    #[error("Cannot parse command: `{0}`")]
    InvalidCommand(String),
    #[error("Cannot convert value {:0} to {1}")]
    ConvertError(Value, &'static str),
    #[error("Cannot process Command {0} with table: {1}, key: {2}. Error: {}")]
    StorageError(&'static str, String, String, String),

    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),
    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Failed to encode frame")]
    FrameError,
    #[error("Failed to encode frame")]
    CertifcateParseError(String, String),
}

impl From<Error> for KvError {
    fn from(e: Error) -> Self {
        KvError::Internal(e.to_string())
    }
}
