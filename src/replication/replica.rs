use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicU16, Ordering},
    },
};

use tokio::sync::RwLock;

use super::Node;

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

    pub async fn replicas_length(&self) -> u16 {
        self.replicas_length.load(Ordering::Acquire)
    }

    pub async fn register_node(&self, node: Node) -> bool {
        let mut rn_guard = self.replica_nodes.write().await;
        if rn_guard
            .insert(node.master_ipaddr().clone(), node)
            .is_some()
        {
            return false;
        }
        self.replicas_length.fetch_add(1, Ordering::AcqRel);

        true
    }

    pub async fn unregister_node(&self, key: SocketAddr) -> bool {
        let mut rn_guard = self.replica_nodes.write().await;
        if rn_guard.remove(&key).is_some() {
            self.replicas_length.fetch_sub(1, Ordering::AcqRel);
            return true;
        }

        false
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
        let result = replica_master.register_node(node_slave).await;

        assert_eq!(result, true, "Should return true when inserting a new node");
        assert_eq!(
            replica_master.replicas_length().await,
            1,
            "Replica length should be 1"
        );
    }

    #[tokio::test]
    async fn test_unregister_node() {
        let node_master = build_node("master", "127.0.0.1", 8000);
        let replica_master = Replica::new(node_master);

        let node_slave = build_node("slave", "127.0.0.1", 8001);
        let slave_addr = node_slave.master_ipaddr().clone();
        replica_master.register_node(node_slave).await;

        assert_eq!(
            replica_master.replicas_length().await,
            1,
            "Replica length should be 1"
        );

        let result = replica_master.unregister_node(slave_addr).await;

        assert_eq!(result, true, "Should return true when removing a node");
        assert_eq!(
            replica_master.replicas_length().await,
            0,
            "Replica length should be 0"
        );
    }
}
