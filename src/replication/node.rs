use std::{
    net::{SocketAddr, SocketAddrV4},
    str::FromStr,
};

use super::ReplicationError;

#[derive(Debug)]
pub enum NodeMode {
    Master,
    Slave,
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

impl NodeMode {
    pub fn is_master(&self) -> bool {
        matches!(self, NodeMode::Master)
    }

    pub fn is_slave(&self) -> bool {
        matches!(self, NodeMode::Slave)
    }
}

#[derive(Debug)]
pub struct Node {
    mode: NodeMode,
    ipaddr: SocketAddr,
}

impl Node {
    pub fn new(mode: String, ip: String, port: u64) -> Result<Self, ReplicationError> {
        let mode = NodeMode::try_from(mode)?;
        let ipaddr = SocketAddr::from_str(format!("{}:{}", ip, port).as_str())?;
        Ok(Self { mode, ipaddr })
    }

    pub fn promote(&mut self, mode: String) -> Result<(), ReplicationError> {
        self.mode = NodeMode::try_from(mode)?;
        Ok(())
    }

    pub fn is_master(&self) -> bool {
        self.mode.is_master()
    }

    pub fn is_slave(&self) -> bool {
        self.mode.is_slave()
    }

    pub fn ipaddr(&self) -> &SocketAddr {
        &self.ipaddr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node_slave() {
        let node = Node::new("slave".into(), "127.0.0.1".into(), 8081)
            .expect("Must create node without errors");

        assert!(node.is_slave(), "Must be a slave node");
        assert_eq!(node.ipaddr().port(), 8081, "Node port does not match");
    }

    #[test]
    fn test_create_node_master() {
        let node = Node::new("master".into(), "127.0.0.1".into(), 8081)
            .expect("Must create node without errors");

        assert!(node.is_master(), "Must be a master node");
        assert_eq!(node.ipaddr().port(), 8081, "Node port does not match");
    }

    #[test]
    fn test_promote_node() {
        let mut node = Node::new("slave".into(), "127.0.0.1".into(), 8081)
            .expect("Must create node without errors");

        assert!(node.is_slave(), "Must be a slave node");
        assert_eq!(node.ipaddr().port(), 8081, "Node port does not match");

        node.promote("master".into())
            .expect("Must promote node without errors");
        assert!(node.is_master(), "Must be a master node");
        assert_eq!(node.ipaddr().port(), 8081, "Node port does not match");
    }
}
