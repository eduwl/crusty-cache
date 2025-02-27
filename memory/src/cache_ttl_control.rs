use std::collections::BTreeMap;
use tokio::sync::RwLock;

pub struct CacheTTLControl {
    items: RwLock<BTreeMap<String, u64>>,
}

impl CacheTTLControl {
    pub fn new() -> Self {
        Self {
            items: RwLock::new(BTreeMap::new()),
        }
    }

    pub async fn insert(&self, store_key: String, timestamp: u64) -> Option<String> {
        let mut items_guard = self.items.write().await;
        if items_guard.get(&store_key).map(|v| v.le(&timestamp)).is_some() {
            return None;
        }

        let updated = items_guard.insert(store_key.to_owned(), timestamp);
        match updated {
            Some(_) => Some(store_key),
            None => None,
        }
    }
}
