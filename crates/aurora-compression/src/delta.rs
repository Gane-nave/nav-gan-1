//! Delta encoding — store differences between consecutive values for time-series data.

/// Delta-encode a sequence of i64 values.
/// The first value is stored as-is; subsequent values are stored as deltas.
pub fn encode_i64(values: &[i64]) -> Vec<i64> {
    if values.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(values.len());
    result.push(values[0]);
    for i in 1..values.len() {
        result.push(values[i] - values[i - 1]);
    }
    result
}

/// Decode delta-encoded i64 values back to original.
pub fn decode_i64(deltas: &[i64]) -> Vec<i64> {
    if deltas.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(deltas.len());
    result.push(deltas[0]);
    for i in 1..deltas.len() {
        result.push(result[i - 1] + deltas[i]);
    }
    result
}

/// Delta-encode a sequence of f64 values (using fixed-point with given precision).
/// Multiplies by 10^precision, rounds, then delta-encodes as i64.
pub fn encode_f64(values: &[f64], precision: u32) -> Vec<i64> {
    let scale = 10f64.powi(precision as i32);
    let fixed: Vec<i64> = values.iter().map(|&v| (v * scale).round() as i64).collect();
    encode_i64(&fixed)
}

/// Decode delta-encoded f64 values.
pub fn decode_f64(deltas: &[i64], precision: u32) -> Vec<f64> {
    let scale = 10f64.powi(precision as i32);
    let fixed = decode_i64(deltas);
    fixed.iter().map(|&v| v as f64 / scale).collect()
}

/// Double-delta encoding for timestamps or monotonically increasing sequences.
/// Stores delta-of-deltas for even better compression of regular intervals.
pub fn encode_double_delta(values: &[i64]) -> Vec<i64> {
    if values.len() <= 1 {
        return values.to_vec();
    }

    let deltas = encode_i64(values);
    if deltas.len() <= 1 {
        return deltas;
    }

    // Store first value and first delta, then delta-of-deltas
    let mut result = Vec::with_capacity(deltas.len());
    result.push(deltas[0]); // first original value
    result.push(deltas[1]); // first delta
    for i in 2..deltas.len() {
        result.push(deltas[i] - deltas[i - 1]);
    }
    result
}

/// Decode double-delta encoded values.
pub fn decode_double_delta(encoded: &[i64]) -> Vec<i64> {
    if encoded.len() <= 1 {
        return encoded.to_vec();
    }

    // Reconstruct deltas first
    let mut deltas = Vec::with_capacity(encoded.len());
    deltas.push(encoded[0]); // first original value
    deltas.push(encoded[1]); // first delta
    for i in 2..encoded.len() {
        deltas.push(deltas[i - 1] + encoded[i]);
    }

    // Then decode deltas to original values
    decode_i64(&deltas)
}

/// Statistics about delta encoding effectiveness.
#[derive(Debug)]
pub struct DeltaStats {
    /// Number of values.
    pub count: usize,
    /// Number of zero deltas (repeated values).
    pub zero_deltas: usize,
    /// Maximum absolute delta.
    pub max_delta: i64,
    /// Average absolute delta.
    pub avg_delta: f64,
}

/// Compute statistics about the delta distribution.
pub fn delta_stats(values: &[i64]) -> DeltaStats {
    let deltas = encode_i64(values);
    let zero_deltas = deltas.iter().skip(1).filter(|&&d| d == 0).count();
    let max_delta = deltas.iter().skip(1).map(|d| d.abs()).max().unwrap_or(0);
    let sum: i64 = deltas.iter().skip(1).map(|d| d.abs()).sum();
    let avg_delta = if deltas.len() > 1 {
        sum as f64 / (deltas.len() - 1) as f64
    } else {
        0.0
    };

    DeltaStats {
        count: values.len(),
        zero_deltas,
        max_delta,
        avg_delta,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64_roundtrip() {
        let values = vec![100, 105, 110, 115, 120];
        let encoded = encode_i64(&values);
        assert_eq!(encoded, vec![100, 5, 5, 5, 5]);
        let decoded = decode_i64(&encoded);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_i64_empty() {
        assert!(encode_i64(&[]).is_empty());
        assert!(decode_i64(&[]).is_empty());
    }

    #[test]
    fn test_i64_single() {
        let values = vec![42];
        let encoded = encode_i64(&values);
        assert_eq!(encoded, vec![42]);
        assert_eq!(decode_i64(&encoded), values);
    }

    #[test]
    fn test_i64_decreasing() {
        let values = vec![100, 90, 80, 70];
        let encoded = encode_i64(&values);
        assert_eq!(encoded, vec![100, -10, -10, -10]);
        assert_eq!(decode_i64(&encoded), values);
    }

    #[test]
    fn test_f64_roundtrip() {
        let values = vec![1.5, 1.6, 1.7, 1.8];
        let encoded = encode_f64(&values, 1);
        let decoded = decode_f64(&encoded, 1);
        for (a, b) in values.iter().zip(decoded.iter()) {
            assert!((a - b).abs() < 0.01);
        }
    }

    #[test]
    fn test_double_delta_regular() {
        // Regular timestamps: 1000, 2000, 3000, 4000, 5000
        let values = vec![1000, 2000, 3000, 4000, 5000];
        let encoded = encode_double_delta(&values);
        // First value = 1000, first delta = 1000, rest are 0 (constant interval)
        assert_eq!(encoded, vec![1000, 1000, 0, 0, 0]);
        let decoded = decode_double_delta(&encoded);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_double_delta_irregular() {
        let values = vec![100, 200, 310, 430];
        let encoded = encode_double_delta(&values);
        let decoded = decode_double_delta(&encoded);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_delta_stats() {
        let values = vec![10, 10, 20, 20, 30];
        let stats = delta_stats(&values);
        assert_eq!(stats.count, 5);
        assert_eq!(stats.zero_deltas, 2);
        assert_eq!(stats.max_delta, 10);
    }

    #[test]
    fn test_delta_stats_single() {
        let stats = delta_stats(&[42]);
        assert_eq!(stats.count, 1);
        assert_eq!(stats.zero_deltas, 0);
    }
}
