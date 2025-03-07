mod node;
mod replica;

pub use node::*;
pub use replica::*;

#[derive(Debug)]
pub enum ReplicationError {
    AddrParseError(String),
    Register(String),
    ParseError(String),
}

impl From<std::net::AddrParseError> for ReplicationError {
    fn from(err: std::net::AddrParseError) -> Self {
        ReplicationError::AddrParseError(err.to_string())
    }
}

impl std::error::Error for ReplicationError {}

impl std::fmt::Display for ReplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReplicationError::AddrParseError(msg) => write!(f, "Address parse error: {}", msg),
            ReplicationError::Register(msg) => write!(f, "Registration: {}", msg),
            ReplicationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}
