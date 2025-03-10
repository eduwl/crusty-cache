mod init_args;
mod node;
mod replica;
mod server;

pub use init_args::*;
pub use node::*;
pub use replica::*;
pub use server::*;

use tokio::task::JoinHandle;

use std::{env, fmt::Display, net::SocketAddr, sync::Arc};

pub fn create_node() -> Result<Node, ReplicationError> {
    let args = INIT_ARGS.get().unwrap();
    let mode = NodeMode::try_from(args.mode.clone())?;
    match mode {
        NodeMode::Master => {
            let ipaddr = "127.0.0.1:50000".parse()?;
            return Ok(Node::new(mode, ipaddr));
        }
        NodeMode::Slave => {
            let ipaddr = format!("{}:{}", args.master_ip, args.port).parse()?;
            return Ok(Node::new(mode, ipaddr));
        }
    }
}

pub async fn start_replication_tasks(
    replica: Arc<Replica>,
) -> Result<Vec<JoinHandle<()>>, ReplicationError> {
    let mut tasks = Vec::new();

    let replication_port = env::var("CR_REPLICATION_PORT").unwrap_or_else(|_| "5555".to_string());

    // Replicação sempre sera ligado ao localhost mesmo sendo um slave
    // Depois melhorar o entendimento e a coesão entre as classes.
    let localhost_with: SocketAddr = format!("127.0.0.1:{}", replication_port).parse()?;
    let replica_task = replica.clone();
    let rp_server_task = tokio::spawn(async move {
        if start_server(replica_task, localhost_with).await.is_err() {
            eprintln!("Failed to start replication server");
        }
    });

    tasks.push(rp_server_task);
    if replica.node.is_master() {
        println!("A conexão cliente para a replicação sera ignorada quando o nó for master");
        return Ok(tasks);
    }

    // Replicação como cliente do slave para o servidor master somente sera
    // iniciado se a replica tiver um nó slave.

    Ok(tasks)
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
