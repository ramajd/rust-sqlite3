use thiserror::Error;

use std::result;

use sqlparser::parser::ParserError;

pub type Result<T> = result::Result<T, SQLRiteError>;

#[derive(Error, Debug, PartialEq)]
pub enum SQLRiteError {
    #[error("Not Implemented error: {0}")]
    NotImplemented(String),
    #[allow(unused)] // TODO: remove after using this type
    #[error("General error: {0}")]
    General(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Unknown command error: {0}")]
    UnknownCommand(String),
    #[error("SQL error: {0:?}")]
    SqlError(#[from] ParserError),
}

#[allow(unused)] // TODO: remove after function used
pub fn sqlrite_error(message: &str) -> SQLRiteError {
    SQLRiteError::General(message.to_owned())
}
