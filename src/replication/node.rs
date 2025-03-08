use std::net::SocketAddr;

use super::{NodeMode, ReplicationError};

#[derive(Debug)]
pub struct Node {
    mode: NodeMode,
    ipaddr: SocketAddr,
}

impl Node {
    pub fn new(mode: NodeMode, ipaddr: SocketAddr) -> Self {
        Self { mode, ipaddr }
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
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_create_node_slave() {
        let node_mode =
            NodeMode::try_from("slave".to_owned()).expect("NodeMode must be master or slave");

        let ipaddr = SocketAddr::from_str(format!("127.0.0.1:{}", 8081).as_str())
            .expect("IpAddr failed to parse");

        let node = Node::new(node_mode, ipaddr);

        assert!(node.is_slave(), "Must be a slave node");
        assert_eq!(node.ipaddr().port(), 8081, "Node port does not match");
    }

    #[test]
    fn test_create_node_master() {
        let node_mode =
            NodeMode::try_from("master".to_owned()).expect("NodeMode must be master or slave");

        let ipaddr = SocketAddr::from_str(format!("127.0.0.1:{}", 8081).as_str())
            .expect("IpAddr failed to parse");

        let node = Node::new(node_mode, ipaddr);

        assert!(node.is_master(), "Must be a master node");
        assert_eq!(node.ipaddr().port(), 8081, "Node port does not match");
    }

    #[test]
    fn test_promote_node() {
        let node_mode =
            NodeMode::try_from("slave".to_owned()).expect("NodeMode must be master or slave");

        let ipaddr = SocketAddr::from_str(format!("127.0.0.1:{}", 8081).as_str())
            .expect("IpAddr failed to parse");

        let mut node = Node::new(node_mode, ipaddr);

        assert!(node.is_slave(), "Must be a slave node");
        assert_eq!(node.ipaddr().port(), 8081, "Node port does not match");

        node.promote("master".into())
            .expect("Must promote node without errors");
        assert!(node.is_master(), "Must be a master node");
        assert_eq!(node.ipaddr().port(), 8081, "Node port does not match");
    }
}
