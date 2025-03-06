use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "response", content = "data")]
pub enum Responses {
    Test(String),
}
