use chrono::Local;
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicU64, Ordering},
};
use tokio::sync::RwLock;

/// Struct do controle do tempo de vida.
///
/// Aqui é onde o tempo de vida é controlado, o mapa contem de forma ordenada
/// os itens mais antigos ao mais novo, o controle é feito separado para não
/// interferir no cache principal e evitar locks prolongados.
///
/// Separando o controle de tempo de vida do cache principal podemos verificar
/// e invalidar valores do cache principal com mais frequencia sem afetar a performance
/// do cache principal utilizado pelos usuarios.
///
/// #Fields:
/// - `length`: AtomicU64 - Contador de itens no cache
/// - `items`: RwLock<BTreeMap<i64, String>> - Mapa ordenado com o tempo de vida
pub struct CacheTTLControl {
    /// Mostra o tamanho atual do mapa retornando o numero de itens.
    length: AtomicU64,
    /// Mapa ordenado pelo tempo de vida junto a key usada no cache principal.
    items: RwLock<BTreeMap<i64, String>>,
}

impl CacheTTLControl {
    pub fn new() -> Self {
        Self {
            length: AtomicU64::new(0),
            items: RwLock::new(BTreeMap::new()),
        }
    }

    /// Retorna o o tamanho atual do mapa.
    pub fn len(&self) -> u64 {
        self.length.load(Ordering::Acquire)
    }

    /// Insere um novo item no mapa contendo o timestamp e a key que esta sendo armazenada no cache principal.
    pub async fn set(&self, timestamp: i64, store_key: String) -> Option<String> {
        let mut items_guard = self.items.write().await;
        let updated = items_guard.insert(timestamp, store_key.to_owned());
        self.length.fetch_add(1, Ordering::AcqRel);
        match updated {
            Some(_) => Some(store_key),
            _ => None,
        }
    }

    /// Limpeza ativa do mapa.
    ///
    /// Limpa ativamente os timestamps expirados comparando com o timestamp atual.
    /// Todo timestamp que estiver incluso no range ate o ponto atual é
    /// mapeado para um vetor e removido do mapa  de controle do tempo de vida.
    ///
    /// Retorna um vetor com as chaves expiradas.
    pub async fn cleanup_expired(&self) -> Option<Vec<String>> {
        let now_timestamp = Local::now().timestamp();
        let mut items_guard = self.items.write().await;

        let expired_keys: Vec<String> = items_guard
            .range(..now_timestamp)
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
        let _200ms = future_point_in_seconds(-1);
        let _300ms = future_point_in_seconds(-2);
        let _10sec = future_point_in_seconds(10);

        let ctc = CacheTTLControl::new();
        ctc.set(_200ms, "value200ms".to_owned()).await;
        assert_eq!(ctc.len(), 1, "Should have a len of 1 after setting");

        ctc.set(_300ms, "value300ms".to_owned()).await;
        assert_eq!(ctc.len(), 2, "Should have a len of 1 after setting");

        ctc.set(_10sec, "value1s".to_owned()).await;
        assert_eq!(ctc.len(), 3, "Should have a len of 1 after setting");

        let expired_keys = ctc.cleanup_expired().await;
        assert_ne!(expired_keys, None, "Should not be None");

        let expired_keys_unwrapped = expired_keys.unwrap();
        assert!(
            &expired_keys_unwrapped.len().eq(&2usize),
            "Should have a len of 2 expired keys"
        );

        assert!(
            !expired_keys_unwrapped.iter().any(|k| k.eq("value1s")),
            "Should not contain value1s"
        );
    }
}
