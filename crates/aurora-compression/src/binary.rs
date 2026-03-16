//! Binary serialization — compact binary format for structured data.

use std::collections::HashMap;

/// Binary data types supported by the serializer.
#[derive(Debug, Clone, PartialEq)]
pub enum BinValue {
    /// Null / no value.
    Null,
    /// Boolean.
    Bool(bool),
    /// Unsigned 8-bit integer.
    U8(u8),
    /// Unsigned 16-bit integer.
    U16(u16),
    /// Unsigned 32-bit integer.
    U32(u32),
    /// Unsigned 64-bit integer.
    U64(u64),
    /// Signed 64-bit integer.
    I64(i64),
    /// 64-bit float.
    F64(f64),
    /// UTF-8 string.
    Str(String),
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// Array of values.
    Array(Vec<BinValue>),
    /// Map of string keys to values.
    Map(Vec<(String, BinValue)>),
}

/// Type tags for binary encoding.
const TAG_NULL: u8 = 0;
const TAG_BOOL_FALSE: u8 = 1;
const TAG_BOOL_TRUE: u8 = 2;
const TAG_U8: u8 = 3;
const TAG_U16: u8 = 4;
const TAG_U32: u8 = 5;
const TAG_U64: u8 = 6;
const TAG_I64: u8 = 7;
const TAG_F64: u8 = 8;
const TAG_STR: u8 = 9;
const TAG_BYTES: u8 = 10;
const TAG_ARRAY: u8 = 11;
const TAG_MAP: u8 = 12;

/// Serialize a BinValue to bytes.
pub fn serialize(value: &BinValue) -> Vec<u8> {
    let mut buf = Vec::new();
    serialize_into(&mut buf, value);
    buf
}

fn serialize_into(buf: &mut Vec<u8>, value: &BinValue) {
    match value {
        BinValue::Null => buf.push(TAG_NULL),
        BinValue::Bool(false) => buf.push(TAG_BOOL_FALSE),
        BinValue::Bool(true) => buf.push(TAG_BOOL_TRUE),
        BinValue::U8(v) => {
            buf.push(TAG_U8);
            buf.push(*v);
        }
        BinValue::U16(v) => {
            buf.push(TAG_U16);
            buf.extend_from_slice(&v.to_le_bytes());
        }
        BinValue::U32(v) => {
            buf.push(TAG_U32);
            buf.extend_from_slice(&v.to_le_bytes());
        }
        BinValue::U64(v) => {
            buf.push(TAG_U64);
            buf.extend_from_slice(&v.to_le_bytes());
        }
        BinValue::I64(v) => {
            buf.push(TAG_I64);
            buf.extend_from_slice(&v.to_le_bytes());
        }
        BinValue::F64(v) => {
            buf.push(TAG_F64);
            buf.extend_from_slice(&v.to_le_bytes());
        }
        BinValue::Str(s) => {
            buf.push(TAG_STR);
            let bytes = s.as_bytes();
            buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            buf.extend_from_slice(bytes);
        }
        BinValue::Bytes(b) => {
            buf.push(TAG_BYTES);
            buf.extend_from_slice(&(b.len() as u32).to_le_bytes());
            buf.extend_from_slice(b);
        }
        BinValue::Array(arr) => {
            buf.push(TAG_ARRAY);
            buf.extend_from_slice(&(arr.len() as u32).to_le_bytes());
            for item in arr {
                serialize_into(buf, item);
            }
        }
        BinValue::Map(entries) => {
            buf.push(TAG_MAP);
            buf.extend_from_slice(&(entries.len() as u32).to_le_bytes());
            for (key, val) in entries {
                let key_bytes = key.as_bytes();
                buf.extend_from_slice(&(key_bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(key_bytes);
                serialize_into(buf, val);
            }
        }
    }
}

/// Deserialize bytes back to a BinValue.
pub fn deserialize(data: &[u8]) -> Option<BinValue> {
    let mut offset = 0;
    deserialize_at(data, &mut offset)
}

fn read_u32_le(data: &[u8], offset: &mut usize) -> Option<u32> {
    if *offset + 4 > data.len() {
        return None;
    }
    let bytes: [u8; 4] = data[*offset..*offset + 4].try_into().ok()?;
    *offset += 4;
    Some(u32::from_le_bytes(bytes))
}

fn deserialize_at(data: &[u8], offset: &mut usize) -> Option<BinValue> {
    if *offset >= data.len() {
        return None;
    }
    let tag = data[*offset];
    *offset += 1;

    match tag {
        TAG_NULL => Some(BinValue::Null),
        TAG_BOOL_FALSE => Some(BinValue::Bool(false)),
        TAG_BOOL_TRUE => Some(BinValue::Bool(true)),
        TAG_U8 => {
            if *offset >= data.len() {
                return None;
            }
            let v = data[*offset];
            *offset += 1;
            Some(BinValue::U8(v))
        }
        TAG_U16 => {
            if *offset + 2 > data.len() {
                return None;
            }
            let bytes: [u8; 2] = data[*offset..*offset + 2].try_into().ok()?;
            *offset += 2;
            Some(BinValue::U16(u16::from_le_bytes(bytes)))
        }
        TAG_U32 => {
            let v = read_u32_le(data, offset)?;
            Some(BinValue::U32(v))
        }
        TAG_U64 => {
            if *offset + 8 > data.len() {
                return None;
            }
            let bytes: [u8; 8] = data[*offset..*offset + 8].try_into().ok()?;
            *offset += 8;
            Some(BinValue::U64(u64::from_le_bytes(bytes)))
        }
        TAG_I64 => {
            if *offset + 8 > data.len() {
                return None;
            }
            let bytes: [u8; 8] = data[*offset..*offset + 8].try_into().ok()?;
            *offset += 8;
            Some(BinValue::I64(i64::from_le_bytes(bytes)))
        }
        TAG_F64 => {
            if *offset + 8 > data.len() {
                return None;
            }
            let bytes: [u8; 8] = data[*offset..*offset + 8].try_into().ok()?;
            *offset += 8;
            Some(BinValue::F64(f64::from_le_bytes(bytes)))
        }
        TAG_STR => {
            let len = read_u32_le(data, offset)? as usize;
            if *offset + len > data.len() {
                return None;
            }
            let s = std::str::from_utf8(&data[*offset..*offset + len]).ok()?;
            *offset += len;
            Some(BinValue::Str(s.to_string()))
        }
        TAG_BYTES => {
            let len = read_u32_le(data, offset)? as usize;
            if *offset + len > data.len() {
                return None;
            }
            let b = data[*offset..*offset + len].to_vec();
            *offset += len;
            Some(BinValue::Bytes(b))
        }
        TAG_ARRAY => {
            let count = read_u32_le(data, offset)? as usize;
            let mut arr = Vec::with_capacity(count);
            for _ in 0..count {
                arr.push(deserialize_at(data, offset)?);
            }
            Some(BinValue::Array(arr))
        }
        TAG_MAP => {
            let count = read_u32_le(data, offset)? as usize;
            let mut entries = Vec::with_capacity(count);
            for _ in 0..count {
                let key_len = read_u32_le(data, offset)? as usize;
                if *offset + key_len > data.len() {
                    return None;
                }
                let key = std::str::from_utf8(&data[*offset..*offset + key_len])
                    .ok()?
                    .to_string();
                *offset += key_len;
                let val = deserialize_at(data, offset)?;
                entries.push((key, val));
            }
            Some(BinValue::Map(entries))
        }
        _ => None,
    }
}

/// Estimate the serialized size of a value without actually serializing.
pub fn estimated_size(value: &BinValue) -> usize {
    match value {
        BinValue::Null | BinValue::Bool(_) => 1,
        BinValue::U8(_) => 2,
        BinValue::U16(_) => 3,
        BinValue::U32(_) => 5,
        BinValue::U64(_) | BinValue::I64(_) | BinValue::F64(_) => 9,
        BinValue::Str(s) => 5 + s.len(),
        BinValue::Bytes(b) => 5 + b.len(),
        BinValue::Array(arr) => 5 + arr.iter().map(estimated_size).sum::<usize>(),
        BinValue::Map(entries) => {
            5 + entries
                .iter()
                .map(|(k, v)| 4 + k.len() + estimated_size(v))
                .sum::<usize>()
        }
    }
}

/// Convert a HashMap to a BinValue::Map.
pub fn from_hashmap(map: &HashMap<String, BinValue>) -> BinValue {
    let entries: Vec<(String, BinValue)> =
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    BinValue::Map(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_roundtrip() {
        let data = serialize(&BinValue::Null);
        assert_eq!(deserialize(&data), Some(BinValue::Null));
    }

    #[test]
    fn test_bool_roundtrip() {
        let data = serialize(&BinValue::Bool(true));
        assert_eq!(deserialize(&data), Some(BinValue::Bool(true)));

        let data = serialize(&BinValue::Bool(false));
        assert_eq!(deserialize(&data), Some(BinValue::Bool(false)));
    }

    #[test]
    fn test_integer_roundtrip() {
        for val in [
            BinValue::U8(255),
            BinValue::U16(60000),
            BinValue::U32(1_000_000),
            BinValue::U64(u64::MAX),
            BinValue::I64(-42),
        ] {
            let data = serialize(&val);
            assert_eq!(deserialize(&data), Some(val));
        }
    }

    #[test]
    fn test_f64_roundtrip() {
        let val = BinValue::F64(3.14159);
        let data = serialize(&val);
        assert_eq!(deserialize(&data), Some(val));
    }

    #[test]
    fn test_string_roundtrip() {
        let val = BinValue::Str("hello world! 🌍".to_string());
        let data = serialize(&val);
        assert_eq!(deserialize(&data), Some(val));
    }

    #[test]
    fn test_bytes_roundtrip() {
        let val = BinValue::Bytes(vec![0, 1, 2, 255]);
        let data = serialize(&val);
        assert_eq!(deserialize(&data), Some(val));
    }

    #[test]
    fn test_array_roundtrip() {
        let val = BinValue::Array(vec![
            BinValue::U8(1),
            BinValue::Str("two".into()),
            BinValue::Bool(true),
        ]);
        let data = serialize(&val);
        assert_eq!(deserialize(&data), Some(val));
    }

    #[test]
    fn test_map_roundtrip() {
        let val = BinValue::Map(vec![
            ("name".into(), BinValue::Str("alice".into())),
            ("age".into(), BinValue::U8(30)),
            ("active".into(), BinValue::Bool(true)),
        ]);
        let data = serialize(&val);
        assert_eq!(deserialize(&data), Some(val));
    }

    #[test]
    fn test_nested_structure() {
        let val = BinValue::Map(vec![(
            "coords".into(),
            BinValue::Array(vec![BinValue::F64(1.5), BinValue::F64(2.5)]),
        )]);
        let data = serialize(&val);
        assert_eq!(deserialize(&data), Some(val));
    }

    #[test]
    fn test_estimated_size() {
        let val = BinValue::U8(42);
        assert_eq!(estimated_size(&val), 2);

        let val = BinValue::Str("hi".into());
        assert_eq!(estimated_size(&val), 7); // 5 + 2
    }

    #[test]
    fn test_invalid_data() {
        assert_eq!(deserialize(&[255]), None); // unknown tag
        assert_eq!(deserialize(&[]), None); // empty
    }

    #[test]
    fn test_from_hashmap() {
        let mut map = HashMap::new();
        map.insert("key".into(), BinValue::U32(42));
        let val = from_hashmap(&map);
        if let BinValue::Map(entries) = &val {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].0, "key");
        } else {
            panic!("expected Map");
        }
    }
}
