use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command", content = "data")]
pub enum Commands {
    Test(String),
}
