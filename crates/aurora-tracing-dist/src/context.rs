//! Trace context — trace IDs, span IDs, and propagation headers.

/// A 128-bit trace ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceId(pub u64, pub u64);

impl TraceId {
    /// Create a new trace ID from two u64 halves.
    pub fn new(high: u64, low: u64) -> Self {
        Self(high, low)
    }

    /// Format as a 32-character hex string.
    pub fn to_hex(&self) -> String {
        format!("{:016x}{:016x}", self.0, self.1)
    }

    /// Parse from a 32-character hex string.
    pub fn from_hex(s: &str) -> Option<Self> {
        if s.len() != 32 {
            return None;
        }
        let high = u64::from_str_radix(&s[..16], 16).ok()?;
        let low = u64::from_str_radix(&s[16..], 16).ok()?;
        Some(Self(high, low))
    }
}

/// A 64-bit span ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanId(pub u64);

impl SpanId {
    /// Create a new span ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Format as a 16-character hex string.
    pub fn to_hex(&self) -> String {
        format!("{:016x}", self.0)
    }

    /// Parse from a 16-character hex string.
    pub fn from_hex(s: &str) -> Option<Self> {
        if s.len() != 16 {
            return None;
        }
        let id = u64::from_str_radix(s, 16).ok()?;
        Some(Self(id))
    }
}

/// Trace flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceFlags(pub u8);

impl TraceFlags {
    /// No flags set.
    pub const NONE: Self = Self(0);
    /// Sampled flag.
    pub const SAMPLED: Self = Self(1);

    /// Check if sampled.
    pub fn is_sampled(&self) -> bool {
        self.0 & 1 != 0
    }
}

/// W3C Trace Context — propagated across service boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceContext {
    /// Trace ID.
    pub trace_id: TraceId,
    /// Parent span ID.
    pub parent_span_id: SpanId,
    /// Trace flags.
    pub flags: TraceFlags,
}

impl TraceContext {
    /// Create a new trace context.
    pub fn new(trace_id: TraceId, parent_span_id: SpanId, flags: TraceFlags) -> Self {
        Self {
            trace_id,
            parent_span_id,
            flags,
        }
    }

    /// Format as W3C traceparent header value.
    /// Format: `00-{trace_id}-{parent_id}-{flags}`
    pub fn to_traceparent(&self) -> String {
        format!(
            "00-{}-{}-{:02x}",
            self.trace_id.to_hex(),
            self.parent_span_id.to_hex(),
            self.flags.0
        )
    }

    /// Parse from W3C traceparent header value.
    pub fn from_traceparent(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 4 || parts[0] != "00" {
            return None;
        }
        let trace_id = TraceId::from_hex(parts[1])?;
        let parent_span_id = SpanId::from_hex(parts[2])?;
        let flags = u8::from_str_radix(parts[3], 16).ok()?;

        Some(Self {
            trace_id,
            parent_span_id,
            flags: TraceFlags(flags),
        })
    }
}

/// Baggage — key-value pairs propagated across service boundaries.
#[derive(Debug, Clone, Default)]
pub struct Baggage {
    items: std::collections::HashMap<String, String>,
}

impl Baggage {
    /// Create empty baggage.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a baggage item.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.items.insert(key.into(), value.into());
    }

    /// Get a baggage item.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.items.get(key).map(|s| s.as_str())
    }

    /// Remove a baggage item.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.items.remove(key)
    }

    /// Number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether baggage is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Format as a header value (comma-separated key=value pairs).
    pub fn to_header(&self) -> String {
        self.items
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Parse from a header value.
    pub fn from_header(s: &str) -> Self {
        let mut baggage = Self::new();
        for pair in s.split(',') {
            let pair = pair.trim();
            if let Some((k, v)) = pair.split_once('=') {
                baggage.set(k.trim(), v.trim());
            }
        }
        baggage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_id_hex_roundtrip() {
        let id = TraceId::new(0x0123456789abcdef, 0xfedcba9876543210);
        let hex = id.to_hex();
        assert_eq!(hex, "0123456789abcdeffedcba9876543210");
        assert_eq!(TraceId::from_hex(&hex), Some(id));
    }

    #[test]
    fn test_trace_id_invalid_hex() {
        assert!(TraceId::from_hex("short").is_none());
        assert!(TraceId::from_hex("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz").is_none());
    }

    #[test]
    fn test_span_id_hex_roundtrip() {
        let id = SpanId::new(0xdeadbeefcafebabe);
        let hex = id.to_hex();
        assert_eq!(hex, "deadbeefcafebabe");
        assert_eq!(SpanId::from_hex(&hex), Some(id));
    }

    #[test]
    fn test_trace_flags() {
        assert!(!TraceFlags::NONE.is_sampled());
        assert!(TraceFlags::SAMPLED.is_sampled());
    }

    #[test]
    fn test_traceparent_roundtrip() {
        let ctx = TraceContext::new(TraceId::new(1, 2), SpanId::new(3), TraceFlags::SAMPLED);
        let header = ctx.to_traceparent();
        let parsed = TraceContext::from_traceparent(&header).unwrap();
        assert_eq!(parsed, ctx);
    }

    #[test]
    fn test_traceparent_format() {
        let ctx = TraceContext::new(TraceId::new(0, 1), SpanId::new(2), TraceFlags::SAMPLED);
        let header = ctx.to_traceparent();
        assert!(header.starts_with("00-"));
        assert!(header.ends_with("-01"));
    }

    #[test]
    fn test_traceparent_invalid() {
        assert!(TraceContext::from_traceparent("invalid").is_none());
        assert!(TraceContext::from_traceparent("01-abc-def-00").is_none()); // wrong version
    }

    #[test]
    fn test_baggage() {
        let mut bag = Baggage::new();
        bag.set("user_id", "42");
        bag.set("region", "us-east");

        assert_eq!(bag.get("user_id"), Some("42"));
        assert_eq!(bag.len(), 2);

        bag.remove("user_id");
        assert!(bag.get("user_id").is_none());
    }

    #[test]
    fn test_baggage_header_roundtrip() {
        let mut bag = Baggage::new();
        bag.set("key1", "val1");
        bag.set("key2", "val2");

        let header = bag.to_header();
        let parsed = Baggage::from_header(&header);
        assert_eq!(parsed.get("key1"), Some("val1"));
        assert_eq!(parsed.get("key2"), Some("val2"));
    }
}
