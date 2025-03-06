use serde::{Deserialize, Serialize};

/// Representa um valor armazenado na memoria ligado a uma chave.
///
/// Todo os dado que entrar sera salvo como bytes e devolvido como bytes.
/// O interessado no dado tera a tarefas de convertelo para o tipo que quiser.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheValue(Vec<u8>);

impl CacheValue {
    /// Inicializa um novo valor com os dados que possuem AsRef implementado
    pub fn new<Value>(input: Value) -> Self
    where
        Value: AsRef<[u8]>,
    {
        CacheValue(input.as_ref().to_vec())
    }

    /// Devolve o valor como referencia a um array de bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::CacheValue;

    #[test]
    fn test_create_cache_value() {
        let value = CacheValue::new("hello");
        let expected = "hello";

        assert_eq!(expected.as_bytes(), value.as_bytes())
    }
}
