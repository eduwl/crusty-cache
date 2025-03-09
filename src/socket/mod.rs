mod commands;
mod responses;
mod server;

use commands::*;
use responses::*;
pub use server::*;

use crate::replication::{Replica, ReplicationError};

use std::fmt::Display;

#[derive(Debug)]
pub enum SocketError {
    Tokio(tokio::io::Error),
}

impl From<tokio::io::Error> for SocketError {
    fn from(error: tokio::io::Error) -> Self {
        SocketError::Tokio(error)
    }
}

impl std::error::Error for SocketError {}

impl Display for SocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketError::Tokio(error) => write!(f, "Socket error: {}", error),
        }
    }
}
