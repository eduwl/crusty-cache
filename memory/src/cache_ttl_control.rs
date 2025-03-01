use chrono::Local;
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicU64, Ordering},
};
use tokio::sync::RwLock;

pub struct CacheTTLControl {
    length: AtomicU64,
    items: RwLock<BTreeMap<i64, String>>,
}

impl CacheTTLControl {
    pub fn new() -> Self {
        Self {
            length: AtomicU64::new(0),
            items: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn len(&self) -> u64 {
        self.length.load(Ordering::Acquire)
    }

    pub async fn set(&self, timestamp: i64, store_key: String) -> Option<String> {
        let mut items_guard = self.items.write().await;
        let updated = items_guard.insert(timestamp, store_key.to_owned());
        self.length.fetch_add(1, Ordering::AcqRel);
        match updated {
            Some(_) => Some(store_key),
            _ => None,
        }
    }

    pub async fn cleanup_expired(&self) -> Option<Vec<String>> {
        let now_tmestamp = Local::now().timestamp();
        let mut items_guard = self.items.write().await;

        let expired_keys: Vec<String> = items_guard
            .range(..now_tmestamp)
            .map(|(_, key)| key.to_owned())
            .collect();

        if expired_keys.len().eq(&0) {
            return None;
        }

        for e_key in expired_keys.iter() {
            items_guard.retain(|_, stored_k| *e_key != *stored_k);
            self.length.fetch_sub(1, Ordering::AcqRel);
        }

        Some(expired_keys)
    }
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, thread::{self, sleep}, time::Duration};

    use chrono::{DateTime, Duration, Local};

    use super::CacheTTLControl;

    fn future_point_in_seconds(seconds: i64) -> i64 {
        let now: DateTime<Local> = Local::now();
        let ttl = Duration::seconds(seconds);
        let future = now + ttl;

        future.timestamp()
    }
    
    fn future_point_in_milis(milis: i64) -> i64 {
        let now: DateTime<Local> = Local::now();
        let ttl = Duration::milliseconds(milis);
        let future = now + ttl;

        future.timestamp()
    }

    #[tokio::test]
    async fn test_insert() {
        let timestamp = future_point_in_milis(500);

        let key = "item-key";
        let ctc = CacheTTLControl::new();
        ctc.set(timestamp, key.to_owned()).await;
        let len = ctc.len();

        assert!(len.eq(&1u64))
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        
        let _200ms = future_point_in_milis(200);
        let _300ms = future_point_in_milis(300);
        let _10sec = future_point_in_seconds(1);
        
        let ctc = CacheTTLControl::new();
        ctc.set(_200ms, "value200ms".to_owned()).await;
        ctc.set(_300ms, "value300ms".to_owned()).await;
        ctc.set(_10sec, "value1s".to_owned()).await;
        
        sleep(Duration::from_secs(1));
        
        let expired_keys = ctc.cleanup_expired().await;
        assert_ne!(expired_keys, None, "Should have a len of 2 expired keys");
        assert
    }
}
