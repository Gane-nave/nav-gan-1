//! Codec — compact binary encoding/decoding using tag-length-value (TLV) format.

use crate::schema::FieldType;

/// A typed value that can be serialized.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
}

impl Value {
    /// Get the field type of this value.
    pub fn field_type(&self) -> FieldType {
        match self {
            Value::Bool(_) => FieldType::Bool,
            Value::U8(_) => FieldType::U8,
            Value::U16(_) => FieldType::U16,
            Value::U32(_) => FieldType::U32,
            Value::U64(_) => FieldType::U64,
            Value::I32(_) => FieldType::I32,
            Value::I64(_) => FieldType::I64,
            Value::F32(_) => FieldType::F32,
            Value::F64(_) => FieldType::F64,
            Value::String(_) => FieldType::String,
            Value::Bytes(_) => FieldType::Bytes,
        }
    }

    /// Encode this value to bytes.
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Value::Bool(v) => vec![if *v { 1 } else { 0 }],
            Value::U8(v) => vec![*v],
            Value::U16(v) => v.to_le_bytes().to_vec(),
            Value::U32(v) => v.to_le_bytes().to_vec(),
            Value::U64(v) => v.to_le_bytes().to_vec(),
            Value::I32(v) => v.to_le_bytes().to_vec(),
            Value::I64(v) => v.to_le_bytes().to_vec(),
            Value::F32(v) => v.to_le_bytes().to_vec(),
            Value::F64(v) => v.to_le_bytes().to_vec(),
            Value::String(s) => {
                let bytes = s.as_bytes();
                let mut result = (bytes.len() as u32).to_le_bytes().to_vec();
                result.extend_from_slice(bytes);
                result
            }
            Value::Bytes(b) => {
                let mut result = (b.len() as u32).to_le_bytes().to_vec();
                result.extend_from_slice(b);
                result
            }
        }
    }

    /// Decode a value from bytes given a field type.
    pub fn decode(field_type: FieldType, data: &[u8]) -> Option<Self> {
        match field_type {
            FieldType::Bool => {
                if data.is_empty() {
                    return None;
                }
                Some(Value::Bool(data[0] != 0))
            }
            FieldType::U8 => {
                if data.is_empty() {
                    return None;
                }
                Some(Value::U8(data[0]))
            }
            FieldType::U16 => {
                let arr: [u8; 2] = data.get(..2)?.try_into().ok()?;
                Some(Value::U16(u16::from_le_bytes(arr)))
            }
            FieldType::U32 => {
                let arr: [u8; 4] = data.get(..4)?.try_into().ok()?;
                Some(Value::U32(u32::from_le_bytes(arr)))
            }
            FieldType::U64 => {
                let arr: [u8; 8] = data.get(..8)?.try_into().ok()?;
                Some(Value::U64(u64::from_le_bytes(arr)))
            }
            FieldType::I32 => {
                let arr: [u8; 4] = data.get(..4)?.try_into().ok()?;
                Some(Value::I32(i32::from_le_bytes(arr)))
            }
            FieldType::I64 => {
                let arr: [u8; 8] = data.get(..8)?.try_into().ok()?;
                Some(Value::I64(i64::from_le_bytes(arr)))
            }
            FieldType::F32 => {
                let arr: [u8; 4] = data.get(..4)?.try_into().ok()?;
                Some(Value::F32(f32::from_le_bytes(arr)))
            }
            FieldType::F64 => {
                let arr: [u8; 8] = data.get(..8)?.try_into().ok()?;
                Some(Value::F64(f64::from_le_bytes(arr)))
            }
            FieldType::String => {
                let len_bytes: [u8; 4] = data.get(..4)?.try_into().ok()?;
                let len = u32::from_le_bytes(len_bytes) as usize;
                let s = std::str::from_utf8(data.get(4..4 + len)?).ok()?;
                Some(Value::String(s.to_string()))
            }
            FieldType::Bytes => {
                let len_bytes: [u8; 4] = data.get(..4)?.try_into().ok()?;
                let len = u32::from_le_bytes(len_bytes) as usize;
                let b = data.get(4..4 + len)?;
                Some(Value::Bytes(b.to_vec()))
            }
            FieldType::Array | FieldType::Map => None, // not directly decodable
        }
    }
}

/// A tagged field in a TLV message.
#[derive(Debug, Clone)]
pub struct TaggedField {
    pub tag: u32,
    pub value: Value,
}

/// Encode a set of tagged fields into a TLV byte stream.
/// Format: [tag: u32 LE][type: u8][length: u32 LE][value: bytes]
pub fn encode_tlv(fields: &[TaggedField]) -> Vec<u8> {
    let mut buf = Vec::new();
    for field in fields {
        let value_bytes = field.value.encode();
        buf.extend_from_slice(&field.tag.to_le_bytes());
        buf.push(type_byte(field.value.field_type()));
        buf.extend_from_slice(&(value_bytes.len() as u32).to_le_bytes());
        buf.extend_from_slice(&value_bytes);
    }
    buf
}

/// Decode a TLV byte stream into tagged fields.
pub fn decode_tlv(data: &[u8]) -> Vec<TaggedField> {
    let mut fields = Vec::new();
    let mut pos = 0;

    while pos + 9 <= data.len() {
        let tag_bytes: [u8; 4] = match data[pos..pos + 4].try_into() {
            Ok(b) => b,
            Err(_) => break,
        };
        let tag = u32::from_le_bytes(tag_bytes);
        let type_id = data[pos + 4];
        let len_bytes: [u8; 4] = match data[pos + 5..pos + 9].try_into() {
            Ok(b) => b,
            Err(_) => break,
        };
        let len = u32::from_le_bytes(len_bytes) as usize;
        pos += 9;

        if pos + len > data.len() {
            break;
        }

        let field_type = match field_type_from_byte(type_id) {
            Some(ft) => ft,
            None => {
                pos += len;
                continue;
            }
        };

        if let Some(value) = Value::decode(field_type, &data[pos..pos + len]) {
            fields.push(TaggedField { tag, value });
        }
        pos += len;
    }

    fields
}

fn type_byte(ft: FieldType) -> u8 {
    match ft {
        FieldType::Bool => 0,
        FieldType::U8 => 1,
        FieldType::U16 => 2,
        FieldType::U32 => 3,
        FieldType::U64 => 4,
        FieldType::I32 => 5,
        FieldType::I64 => 6,
        FieldType::F32 => 7,
        FieldType::F64 => 8,
        FieldType::String => 9,
        FieldType::Bytes => 10,
        FieldType::Array => 11,
        FieldType::Map => 12,
    }
}

fn field_type_from_byte(b: u8) -> Option<FieldType> {
    match b {
        0 => Some(FieldType::Bool),
        1 => Some(FieldType::U8),
        2 => Some(FieldType::U16),
        3 => Some(FieldType::U32),
        4 => Some(FieldType::U64),
        5 => Some(FieldType::I32),
        6 => Some(FieldType::I64),
        7 => Some(FieldType::F32),
        8 => Some(FieldType::F64),
        9 => Some(FieldType::String),
        10 => Some(FieldType::Bytes),
        11 => Some(FieldType::Array),
        12 => Some(FieldType::Map),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_roundtrip() {
        let v = Value::Bool(true);
        let encoded = v.encode();
        let decoded = Value::decode(FieldType::Bool, &encoded).unwrap();
        assert_eq!(decoded, Value::Bool(true));
    }

    #[test]
    fn test_u32_roundtrip() {
        let v = Value::U32(42);
        let encoded = v.encode();
        let decoded = Value::decode(FieldType::U32, &encoded).unwrap();
        assert_eq!(decoded, Value::U32(42));
    }

    #[test]
    fn test_f64_roundtrip() {
        let v = Value::F64(1.23456);
        let encoded = v.encode();
        let decoded = Value::decode(FieldType::F64, &encoded).unwrap();
        assert_eq!(decoded, Value::F64(1.23456));
    }

    #[test]
    fn test_string_roundtrip() {
        let v = Value::String("hello world".into());
        let encoded = v.encode();
        let decoded = Value::decode(FieldType::String, &encoded).unwrap();
        assert_eq!(decoded, Value::String("hello world".into()));
    }

    #[test]
    fn test_bytes_roundtrip() {
        let v = Value::Bytes(vec![1, 2, 3, 4, 5]);
        let encoded = v.encode();
        let decoded = Value::decode(FieldType::Bytes, &encoded).unwrap();
        assert_eq!(decoded, Value::Bytes(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_i64_roundtrip() {
        let v = Value::I64(-9999);
        let encoded = v.encode();
        let decoded = Value::decode(FieldType::I64, &encoded).unwrap();
        assert_eq!(decoded, Value::I64(-9999));
    }

    #[test]
    fn test_tlv_encode_decode() {
        let fields = vec![
            TaggedField {
                tag: 1,
                value: Value::F64(32.0),
            },
            TaggedField {
                tag: 2,
                value: Value::F64(-117.0),
            },
            TaggedField {
                tag: 3,
                value: Value::String("test".into()),
            },
        ];

        let encoded = encode_tlv(&fields);
        let decoded = decode_tlv(&encoded);

        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded[0].tag, 1);
        assert_eq!(decoded[0].value, Value::F64(32.0));
        assert_eq!(decoded[1].tag, 2);
        assert_eq!(decoded[1].value, Value::F64(-117.0));
        assert_eq!(decoded[2].tag, 3);
        assert_eq!(decoded[2].value, Value::String("test".into()));
    }

    #[test]
    fn test_tlv_empty() {
        let encoded = encode_tlv(&[]);
        assert!(encoded.is_empty());
        let decoded = decode_tlv(&encoded);
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_value_field_type() {
        assert_eq!(Value::Bool(true).field_type(), FieldType::Bool);
        assert_eq!(Value::F64(1.0).field_type(), FieldType::F64);
        assert_eq!(Value::String("x".into()).field_type(), FieldType::String);
    }

    #[test]
    fn test_decode_truncated_data() {
        assert!(Value::decode(FieldType::U32, &[1, 2]).is_none());
        assert!(Value::decode(FieldType::F64, &[1, 2, 3]).is_none());
    }
}
