use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use dashmap::DashMap;

use super::cache_value::CacheValue;

/// Struct Gerenciadora do Cache.
///
/// Esse é o ponto principal da existencia dos dados no cache
/// e sera acessado globalmente pelo serviço.
///
/// # Fields
/// ```
/// length: AtomicU64
/// memory_map: Arc<DashMap<String, DataValue>>
/// ```
pub struct Store {
    /// Mostra o tamanho do cache atualmente retornando o numero de itens.
    length: AtomicU64,
    /// Cache em memoria usando DashMap para uma abordagem mais limpa
    /// enquanto mantem Safe Thread e imutabilidade local.
    memory_map: Arc<DashMap<String, CacheValue>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            length: AtomicU64::new(0),
            memory_map: Arc::new(DashMap::new()),
        }
    }

    /// Busca o tamanho atual do mapa na memória.
    pub fn len(&self) -> u64 {
        self.length.load(Ordering::Acquire)
    }

    /// Busca um item no cache com base em uma `key`.
    ///
    /// O mapa ira criar um guard protegendo a referencia ate o fim dessa função,
    /// o clone irá garantir que receberemos uma copia do valor valida enquanto o
    /// guard é dropado.
    pub fn find(&self, key: &str) -> Option<CacheValue> {
        self.memory_map.get(key).map(|guard| guard.value().clone())
    }

    /// Insere um novo valor no cache enquanto aumenta o tamanho de length.
    pub fn insert(&self, key: String, value: CacheValue) {
        self.memory_map.insert(key, value);
        self.length.fetch_add(1, Ordering::AcqRel);
    }

    /// Remove um valor no cache, se for encontrado um valor com a `key`, é diminuido o valor de length.
    pub fn delete(&self, key: &str) {
        if self.memory_map.remove(key).is_some() {
            self.length.fetch_sub(1, Ordering::AcqRel);
        };
    }

    /// Limpa todas as `key/value` da memoria e zera o valor de length.
    pub fn clear(&self) {
        self.memory_map.clear();
        self.length.store(0, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_insert() {
        let store = Store::new();

        let key = "test_key";
        let value = CacheValue::new("value");

        store.insert(key.to_string(), value.clone());
        let hit = store.find(key).unwrap();

        // Same value as CacheValue
        assert_eq!(hit, value);

        // Same raw bytes
        assert_eq!("value".as_bytes(), hit.as_bytes());
    }

    #[test]
    fn test_multi_inserts() {
        let store = Arc::new(Store::new());

        // Set Data
        thread::scope(|s| {
            let mut tasks = vec![];
            for i in 0..100 {
                let store = store.clone();
                let task = s.spawn(move || {
                    let value = CacheValue::new(format!("value-{i}"));
                    store.insert(format!("key-{}", i), value);
                });
                tasks.push(task);
            }

            for task in tasks {
                let _ = task.join();
            }
        });

        for i in 0..100 {
            let key = format!("key-{}", i);
            let string_value = format!("value-{}", i);
            let value = CacheValue::new(string_value.clone());

            let hit = store.find(&key).unwrap();

            // Same value as CacheValue
            assert_eq!(hit, value);

            // Same raw bytes
            assert_eq!(string_value.as_bytes(), hit.as_bytes());
        }
    }

    #[test]
    fn test_delete() {
        let store = Arc::new(Store::new());

        let inserted = 100;
        // Set Data
        thread::scope(|s| {
            let mut tasks = vec![];
            for i in 0..inserted {
                let store = store.clone();
                let task = s.spawn(move || {
                    let value = CacheValue::new(format!("value-{i}"));
                    store.insert(format!("key-{}", i), value);
                });
                tasks.push(task);
            }

            for task in tasks {
                let _ = task.join();
            }
        });

        let removed = 10;
        for i in 0..removed {
            let key = format!("key-{}", i);
            store.delete(&key);
        }

        assert_eq!((inserted - removed), store.len())
    }

    #[test]
    fn test_cleanup_cache() {
        let store = Arc::new(Store::new());

        let inserted = 100;
        // Set Data
        thread::scope(|s| {
            let mut tasks = vec![];
            for i in 0..inserted {
                let store = store.clone();
                let task = s.spawn(move || {
                    let value = CacheValue::new(format!("value-{i}"));
                    store.insert(format!("key-{}", i), value);
                });
                tasks.push(task);
            }

            for task in tasks {
                let _ = task.join();
            }
        });

        store.clear();

        assert_eq!(0, store.len())
    }
}
