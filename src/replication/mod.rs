mod init_args;
mod node;
mod replica;
mod server;

pub use init_args::*;
pub use node::*;
pub use replica::*;
pub use server::*;

use std::fmt::Display;

pub fn create_node() -> Result<Node, ReplicationError> {
    let args = INIT_ARGS.get().unwrap();
    let mode = NodeMode::try_from(args.mode.clone())?;
    let ipaddr = format!("{}:{}", args.master_ip, args.port).parse()?;
    Ok(Node::new(mode, ipaddr))
}

#[derive(Debug)]
pub enum ReplicationError {
    AddrParseError(String),
    Register(String),
    ParseError(String),
    Tokio(tokio::io::Error),
    Unregister(String),
}

impl From<std::net::AddrParseError> for ReplicationError {
    fn from(err: std::net::AddrParseError) -> Self {
        ReplicationError::AddrParseError(err.to_string())
    }
}

impl From<tokio::io::Error> for ReplicationError {
    fn from(err: tokio::io::Error) -> Self {
        ReplicationError::Tokio(err)
    }
}

impl std::error::Error for ReplicationError {}

impl std::fmt::Display for ReplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReplicationError::AddrParseError(msg) => write!(f, "Address parse error: {}", msg),
            ReplicationError::Register(msg) => write!(f, "Register error: {}", msg),
            ReplicationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ReplicationError::Tokio(msg) => write!(f, "Tokio error: {}", msg),
            ReplicationError::Unregister(msg) => write!(f, "Unregister error: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum NodeMode {
    Master,
    Slave,
}

impl NodeMode {
    pub fn is_master(&self) -> bool {
        matches!(self, NodeMode::Master)
    }

    pub fn is_slave(&self) -> bool {
        matches!(self, NodeMode::Slave)
    }
}

impl TryFrom<String> for NodeMode {
    type Error = ReplicationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "master" => Ok(NodeMode::Master),
            "slave" => Ok(NodeMode::Slave),
            _ => Err(ReplicationError::ParseError(
                "invalid replica mode".to_string(),
            )),
        }
    }
}

impl Display for NodeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode_str = match self {
            NodeMode::Master => "master",
            NodeMode::Slave => "slave",
        };
        write!(f, "{}", mode_str)
    }
}
