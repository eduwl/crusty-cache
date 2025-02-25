use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheValue(Vec<u8>);

impl CacheValue {
    pub fn new<T: AsRef<[u8]>>(input: T) -> Self {
        CacheValue(input.as_ref().to_vec())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_create_cache_value() {}
}
