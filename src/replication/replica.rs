use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicU16, Ordering},
    },
};

use tokio::sync::RwLock;

use super::{Node, ReplicationError};

pub struct Replica {
    pub node: Node,
    replicas_length: AtomicU16,
    replica_nodes: Arc<RwLock<HashMap<SocketAddr, Node>>>,
}

impl Replica {
    pub fn new(node: Node) -> Self {
        Self {
            node,
            replicas_length: AtomicU16::new(0),
            replica_nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn relicas_length(&self) -> u16 {
        self.replicas_length.load(Ordering::Acquire)
    }

    pub async fn register_node(&self, node: Node) -> Result<bool, ReplicationError> {
        if self.node.is_slave() {
            return Err(ReplicationError::Register(
                "Slave node cannot register a replica".to_string(),
            ));
        }

        let mut rn_guard = self.replica_nodes.write().await;
        if rn_guard.insert(node.ipaddr().clone(), node).is_some() {
            return Ok(false);
        }
        self.replicas_length.fetch_add(1, Ordering::AcqRel);

        Ok(true)
    }

    pub async fn unregister_node(&self, key: SocketAddr) -> Result<bool, ReplicationError> {
        if self.node.is_slave() {
            return Err(ReplicationError::Unregister(
                "Slave node cannot unregister a replica".to_string(),
            ));
        }

        let mut rn_guard = self.replica_nodes.write().await;
        if rn_guard.remove(&key).is_some() {
            self.replicas_length.fetch_sub(1, Ordering::AcqRel);
            return Ok(true);
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {

    use crate::replication::NodeMode;

    use super::{Node, Replica, SocketAddr};

    fn build_node(mode: &str, ip: &str, port: u64) -> Node {
        let node_mode =
            NodeMode::try_from(mode.to_owned()).expect("NodeMode must be master or slave");
        let ipaddr: SocketAddr = format!("{}:{}", ip, port)
            .parse()
            .expect("IpAddr failed to parse");
        Node::new(node_mode, ipaddr)
    }

    #[tokio::test]
    async fn test_register_node() {
        let node_master = build_node("master", "127.0.0.1", 8000);
        let replica_master = Replica::new(node_master);

        let node_slave = build_node("slave", "127.0.0.1", 8001);
        let result = replica_master
            .register_node(node_slave)
            .await
            .expect("Failed to register node");

        assert_eq!(result, true, "Should return true when inserting a new node");
        assert_eq!(
            replica_master.relicas_length().await,
            1,
            "Replica length should be 1"
        );
    }

    #[tokio::test]
    async fn test_register_node_as_slave() {
        let node_slave = build_node("slave", "127.0.0.1", 8001);
        let replica_slave = Replica::new(node_slave);

        let node_slave2 = build_node("slave", "127.0.0.1", 8002);
        let result = replica_slave.register_node(node_slave2).await;

        assert!(result.is_err(), "Should return an error when inserting a new node");
    }

    #[tokio::test]
    async fn test_unregister_node() {
        let node_master = build_node("master", "127.0.0.1", 8000);
        let replica_master = Replica::new(node_master);

        let node_slave = build_node("slave", "127.0.0.1", 8001);
        let slave_addr = node_slave.ipaddr().clone();
        replica_master
            .register_node(node_slave)
            .await
            .expect("Failed to register node");

        assert_eq!(
            replica_master.relicas_length().await,
            1,
            "Replica length should be 1"
        );

        let result = replica_master
            .unregister_node(slave_addr)
            .await
            .expect("Failed to unregister node");

        assert_eq!(result, true, "Should return true when removing a node");
        assert_eq!(
            replica_master.relicas_length().await,
            0,
            "Replica length should be 0"
        );
    }

    #[tokio::test]
    async fn test_unregister_node_as_slave() {
        let node_slave = build_node("slave", "127.0.0.1", 8001);
        let replica_slave = Replica::new(node_slave);

        let node_slave2 = build_node("slave", "127.0.0.1", 8002);
        let slave_addr = node_slave2.ipaddr().clone();

        let result = replica_slave.unregister_node(slave_addr).await;

        assert!(result.is_err(), "Should return an error when removing a node");
    }
}
