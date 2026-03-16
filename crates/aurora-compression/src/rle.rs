//! Run-Length Encoding (RLE) — simple lossless compression for repetitive data.

/// A single RLE run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    /// The byte value.
    pub value: u8,
    /// Number of consecutive occurrences.
    pub count: u32,
}

/// Encode data using run-length encoding.
pub fn encode(data: &[u8]) -> Vec<Run> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut runs = Vec::new();
    let mut current = data[0];
    let mut count = 1u32;

    for &byte in &data[1..] {
        if byte == current && count < u32::MAX {
            count += 1;
        } else {
            runs.push(Run {
                value: current,
                count,
            });
            current = byte;
            count = 1;
        }
    }
    runs.push(Run {
        value: current,
        count,
    });

    runs
}

/// Decode RLE-encoded data back to original bytes.
pub fn decode(runs: &[Run]) -> Vec<u8> {
    let mut result = Vec::new();
    for run in runs {
        for _ in 0..run.count {
            result.push(run.value);
        }
    }
    result
}

/// Compression ratio (compressed_size / original_size).
/// Each Run is stored as 5 bytes (1 value + 4 count).
pub fn compression_ratio(original_len: usize, runs: &[Run]) -> f64 {
    if original_len == 0 {
        return 1.0;
    }
    let compressed_size = runs.len() * 5;
    compressed_size as f64 / original_len as f64
}

/// Encode data to a compact byte representation.
/// Format: [value: u8, count_high: u8, count_low: u8] per run (3 bytes per run, max count 65535).
pub fn encode_bytes(data: &[u8]) -> Vec<u8> {
    let runs = encode(data);
    let mut result = Vec::with_capacity(runs.len() * 3);
    for run in &runs {
        let count = run.count.min(65535) as u16;
        result.push(run.value);
        result.push((count >> 8) as u8);
        result.push((count & 0xFF) as u8);
    }
    result
}

/// Decode compact byte representation back to original data.
pub fn decode_bytes(encoded: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;
    while i + 2 < encoded.len() {
        let value = encoded[i];
        let count = ((encoded[i + 1] as u16) << 8) | encoded[i + 2] as u16;
        for _ in 0..count {
            result.push(value);
        }
        i += 3;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_simple() {
        let data = b"aaabbbcccc";
        let runs = encode(data);
        assert_eq!(runs.len(), 3);
        assert_eq!(
            runs[0],
            Run {
                value: b'a',
                count: 3
            }
        );
        assert_eq!(
            runs[1],
            Run {
                value: b'b',
                count: 3
            }
        );
        assert_eq!(
            runs[2],
            Run {
                value: b'c',
                count: 4
            }
        );

        let decoded = decode(&runs);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_encode_empty() {
        let runs = encode(b"");
        assert!(runs.is_empty());
        assert!(decode(&runs).is_empty());
    }

    #[test]
    fn test_encode_single_byte() {
        let data = b"x";
        let runs = encode(data);
        assert_eq!(runs.len(), 1);
        assert_eq!(decode(&runs), data);
    }

    #[test]
    fn test_encode_no_repetition() {
        let data = b"abcdef";
        let runs = encode(data);
        assert_eq!(runs.len(), 6); // worst case: each byte is a run
        assert_eq!(decode(&runs), data);
    }

    #[test]
    fn test_encode_all_same() {
        let data = vec![0xFF; 1000];
        let runs = encode(&data);
        assert_eq!(runs.len(), 1);
        assert_eq!(
            runs[0],
            Run {
                value: 0xFF,
                count: 1000
            }
        );
        assert_eq!(decode(&runs), data);
    }

    #[test]
    fn test_compression_ratio() {
        let data = vec![0x00; 100];
        let runs = encode(&data);
        let ratio = compression_ratio(data.len(), &runs);
        assert!(ratio < 0.1); // highly compressible
    }

    #[test]
    fn test_byte_encoding_roundtrip() {
        let data = b"aaabbbcccdddd";
        let encoded = encode_bytes(data);
        let decoded = decode_bytes(&encoded);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_byte_encoding_binary() {
        let data = vec![0u8; 500];
        let encoded = encode_bytes(&data);
        assert!(encoded.len() < data.len());
        let decoded = decode_bytes(&encoded);
        assert_eq!(decoded, data);
    }
}
