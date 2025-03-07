use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use super::{Node, ReplicationError};

pub struct Replica {
    uuid: Uuid,
    pub node: Node,
    pub replica_nodes: Arc<RwLock<HashMap<SocketAddr, Node>>>,
}

impl Replica {
    pub fn new(node: Node) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            node,
            replica_nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_replica(&self, node: Node) -> Result<(), ReplicationError> {
        if self.node.is_slave() {
            return Err(ReplicationError::Register(
                "Slave node cannot register a replica".to_string(),
            ));
        }

        let mut rn_guard = self.replica_nodes.write().await;
        rn_guard.insert(*node.ipaddr(), node);
        
        Ok(())
    }
}
