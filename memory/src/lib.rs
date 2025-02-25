use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

pub mod data_value;
pub mod store;

#[derive(Debug)]
pub enum DataValueError {
    BoolError(ParseBoolError),
    FloatError(ParseFloatError),
    IntError(ParseIntError),
    InvalidType(String),
    JsonError(serde_json::Error),
}

impl From<ParseBoolError> for DataValueError {
    fn from(value: ParseBoolError) -> Self {
        DataValueError::BoolError(value)
    }
}

impl From<ParseFloatError> for DataValueError {
    fn from(value: ParseFloatError) -> Self {
        DataValueError::FloatError(value)
    }
}

impl From<ParseIntError> for DataValueError {
    fn from(value: ParseIntError) -> Self {
        DataValueError::IntError(value)
    }
}

impl From<serde_json::Error> for DataValueError {
    fn from(value: serde_json::Error) -> Self {
        DataValueError::JsonError(value)
    }
}

impl fmt::Display for DataValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataValueError::BoolError(err) => write!(f, "{:?}", err),
            DataValueError::FloatError(err) => write!(f, "{:?}", err),
            DataValueError::IntError(err) => write!(f, "{:?}", err),
            DataValueError::InvalidType(err) => write!(f, "{:?}", err),
            DataValueError::JsonError(err) => write!(f, "{:?}", err),
        }
    }
}

impl std::error::Error for DataValueError {}
