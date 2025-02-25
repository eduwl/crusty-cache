use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use super::DataValueError;

/// Enumerador com os tipos que sera usado para armazenar os dados na memoria.
///
/// Tenta otimizar o tipo de dado para fazer um uso mais conservador da mesmoria
/// e nao ocupar mais bytes alem do necessario.
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Array(Array),
    Number(Number),
    String(String),
}

impl Serialize for DataValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DataValue::String(s) => serializer.serialize_str(s),
            DataValue::Number(n) => n.serialize(serializer),
            DataValue::Array(a) => a.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DataValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Tenta desserializar como uma string primeiro
        if let Ok(s) = String::deserialize(deserializer) {
            return Ok(DataValue::String(s));
        }

        // Se não for uma string, tenta desserializar como um número ou array
        // (implemente a lógica para Number e Array aqui)
        Err(de::Error::custom("Failed to deserialize DataValue"))
    }
}

impl DataValue {
    /// Tenta inferir com base em uma string que vem do usuario qual o tipo de dados
    /// correto para usar ao salvar no cache, caso houver algum problema com
    pub fn infer(input: &str) -> Result<Self, DataValueError> {
        match serde_json::from_str::<Value>(input)? {
            Value::String(s) => Ok(DataValue::String(s)),
            Value::Bool(b) => Ok(DataValue::String(b.to_string())),
            Value::Number(n) => {
                let value = n.as_u128().ok_or(DataValueError::InvalidType(
                    "Não foi possivel parsear o valor numerico como u128.".to_string(),
                ))?;

                if value <= u8::MAX as u128 {
                    return Ok(DataValue::Number((value as u8).into()));
                }

                if value <= u16::MAX as u128 {
                    return Ok(DataValue::Number((value as u16).into()));
                }

                if value <= u32::MAX as u128 {
                    return Ok(DataValue::Number((value as u32).into()));
                }

                if value <= u64::MAX as u128 {
                    return Ok(DataValue::Number((value as u64).into()));
                }

                if value <= u128::MAX {
                    return Ok(DataValue::Number(value.into()));
                }

                let value = n.as_i128().ok_or(DataValueError::InvalidType(
                    "Não foi possivel parsear o valor numerico como i128.".to_string(),
                ))?;

                if value >= i8::MIN as i128 && value <= i8::MAX as i128 {
                    return Ok(DataValue::Number(value.into()));
                }

                if value >= i16::MIN as i128 && value <= i16::MAX as i128 {
                    return Ok(DataValue::Number(value.into()));
                }

                if value >= i32::MIN as i128 && value <= i32::MAX as i128 {
                    return Ok(DataValue::Number(value.into()));
                }

                if value >= i64::MIN as i128 && value <= i64::MAX as i128 {
                    return Ok(DataValue::Number(value.into()));
                }

                if value >= i128::MIN && value <= i128::MAX {
                    return Ok(DataValue::Number(value.into()));
                }

                Err(DataValueError::InvalidType(
                    "Não foi possivel determinar o valor numerico".to_string(),
                ))
            }
            Value::Array(vector) => {
                // unsigned
                if vector
                    .iter()
                    .all(|v| v.as_u64().map(|n| n <= u8::MAX as u64).unwrap_or(false))
                {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|v| Number::U8(v.as_u64().unwrap_or(0) as u8))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                if vector
                    .iter()
                    .all(|v| v.as_u64().map(|n| n <= u16::MAX as u64).unwrap_or(false))
                {
                    let values = vector
                        .iter()
                        .map(|v| Number::U16(v.as_u64().unwrap_or(0) as u16))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                if vector
                    .iter()
                    .all(|v| v.as_u64().map(|n| n <= u32::MAX as u64).unwrap_or(false))
                {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|v| Number::U32(v.as_u64().unwrap_or(0) as u32))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                if vector
                    .iter()
                    .all(|v| v.as_u64().map(|n| n <= u64::MAX).unwrap_or(false))
                {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|n| Number::U64(n.as_u64().unwrap_or(0)))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                if vector
                    .iter()
                    .all(|v| v.as_u64().map(|n| n as u128 <= u128::MAX).unwrap_or(false))
                {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|n| Number::U128(n.as_u64().unwrap_or(0) as u128))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                // signed
                if vector.iter().all(|v| {
                    v.as_i64()
                        .map(|n| n >= i8::MIN as i64 && n <= i8::MAX as i64)
                        .unwrap_or(false)
                }) {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|v| Number::I8(v.as_i64().unwrap_or(0) as i8))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                if vector.iter().all(|v| {
                    v.as_i64()
                        .map(|n| n >= i16::MIN as i64 && n <= i16::MAX as i64)
                        .unwrap_or(false)
                }) {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|v| Number::I16(v.as_i64().unwrap_or(0) as i16))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                if vector.iter().all(|v| {
                    v.as_i64()
                        .map(|n| n >= i32::MIN as i64 && n <= i32::MAX as i64)
                        .unwrap_or(false)
                }) {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|v| Number::I32(v.as_i64().unwrap_or(0) as i32))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                if vector.iter().all(|v| {
                    v.as_i64()
                        .map(|n| n >= i64::MIN && n <= i64::MAX)
                        .unwrap_or(false)
                }) {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|v| Number::I64(v.as_i64().unwrap_or(0)))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                if vector.iter().all(|v| {
                    v.as_i64()
                        .map(|n| n as i128 >= i128::MIN && n as i128 <= i128::MAX)
                        .unwrap_or(false)
                }) {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|v| Number::I128(v.as_i64().unwrap_or(0) as i128))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                // float
                if vector.iter().all(|v| {
                    v.as_f64()
                        .map(|n| n >= f64::MIN && n <= f64::MAX)
                        .unwrap_or(false)
                }) {
                    let values: Vec<Number> = vector
                        .iter()
                        .map(|v| Number::Float(v.as_f64().unwrap_or(0f64) as f64))
                        .collect();

                    return Ok(DataValue::Array(Array::Number(values)));
                }

                let values: Vec<String> = vector.into_iter().map(|v| v.to_string()).collect();
                return Ok(DataValue::Array(Array::Strings(values)));
            }
            _ => Ok(DataValue::String(input.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Array {
    Number(Vec<Number>),
    Strings(Vec<String>),
}

//# ################################################################################ #
//#                                 From DataValue                                   #
//# ################################################################################ #

impl From<Array> for DataValue {
    fn from(value: Array) -> Self {
        DataValue::Array(value)
    }
}

impl From<Number> for DataValue {
    fn from(value: Number) -> Self {
        DataValue::Number(value)
    }
}

impl From<String> for DataValue {
    fn from(value: String) -> Self {
        DataValue::String(value)
    }
}

//# ################################################################################ #
//#                                 From Number                                      #
//# ################################################################################ #
impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::U8(value)
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::U16(value)
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::U32(value)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::U64(value)
    }
}

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Number::U128(value)
    }
}

//

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::I8(value)
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::I16(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::I32(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::I64(value)
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Number::I128(value)
    }
}

#[cfg(test)]
mod test {
    use crate::data_value::Array;

    use super::DataValue;

    #[test]
    fn test_infer_from_string() {
        let input = "\"hello\"";

        let result = DataValue::infer(input).unwrap();
        assert_eq!(result, DataValue::String("hello".to_string()));

        let serialize = serde_json::to_string_pretty(&result).unwrap();
        assert_eq!(input, serialize)
    }

    #[test]
    fn test_infer_from_vec_string() {
        let input = r#"["a", "b", "c"]"#;

        let result = DataValue::infer(input).unwrap();
        assert_eq!(
            result,
            DataValue::Array(Array::Strings(vec![
                "\"a\"".to_string(),
                "\"b\"".to_string(),
                "\"c\"".to_string()
            ]))
        );

        let serialize = serde_json::to_string(&result).unwrap();
        assert_eq!(input, serialize);
    }
}
