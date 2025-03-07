mod node;

pub use node::*;

pub enum ReplicationError {
    ParseError(String),
}
