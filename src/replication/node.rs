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
    port: String,
}

impl Node {
    pub fn new(mode: String, port: String) -> Result<Self, ReplicationError> {
        let mode = NodeMode::try_from(mode)?;
        Ok(Self { mode, port })
    }
}
