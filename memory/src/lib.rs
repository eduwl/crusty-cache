pub mod store;

#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    String(String),
    List(ListType),
}

impl DataValue {
    pub fn infer_value(input: &str) -> Self {
        DataValue::String(input.to_string())
    }
}

impl From<String> for DataValue {
    fn from(value: String) -> Self {
        DataValue::String(value)
    }
}

impl From<ListType> for DataValue {
    fn from(value: ListType) -> Self {
        DataValue::List(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListType {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    U128(Vec<u128>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    I128(Vec<i128>),
    Strings(Vec<String>),
}

impl From<Vec<u8>> for ListType {
    fn from(value: Vec<u8>) -> Self {
        ListType::U8(value)
    }
}

impl From<Vec<u16>> for ListType {
    fn from(value: Vec<u16>) -> Self {
        ListType::U16(value)
    }
}

impl From<Vec<u32>> for ListType {
    fn from(value: Vec<u32>) -> Self {
        ListType::U32(value)
    }
}

impl From<Vec<u64>> for ListType {
    fn from(value: Vec<u64>) -> Self {
        ListType::U64(value)
    }
}

impl From<Vec<u128>> for ListType {
    fn from(value: Vec<u128>) -> Self {
        ListType::U128(value)
    }
}

impl From<Vec<i8>> for ListType {
    fn from(value: Vec<i8>) -> Self {
        ListType::I8(value)
    }
}

impl From<Vec<i16>> for ListType {
    fn from(value: Vec<i16>) -> Self {
        ListType::I16(value)
    }
}

impl From<Vec<i32>> for ListType {
    fn from(value: Vec<i32>) -> Self {
        ListType::I32(value)
    }
}

impl From<Vec<i64>> for ListType {
    fn from(value: Vec<i64>) -> Self {
        ListType::I64(value)
    }
}

impl From<Vec<i128>> for ListType {
    fn from(value: Vec<i128>) -> Self {
        ListType::I128(value)
    }
}

impl From<Vec<String>> for ListType {
    fn from(value: Vec<String>) -> Self {
        ListType::Strings(value)
    }
}
