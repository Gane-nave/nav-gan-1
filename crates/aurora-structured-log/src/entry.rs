//! Structured log entries with fields, levels, and timestamps.

use std::collections::HashMap;
use std::fmt;

/// Log severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    /// Verbose debugging information.
    Trace,
    /// Debugging information.
    Debug,
    /// General information.
    Info,
    /// Warning conditions.
    Warn,
    /// Error conditions.
    Error,
    /// Critical/fatal conditions.
    Fatal,
}

impl LogLevel {
    /// Numeric severity (higher = more severe).
    pub fn severity(&self) -> u8 {
        match self {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
            LogLevel::Fatal => 5,
        }
    }

    /// Parse from string (case-insensitive).
    pub fn from_str_ci(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "trace" => Some(LogLevel::Trace),
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "warn" | "warning" => Some(LogLevel::Warn),
            "error" | "err" => Some(LogLevel::Error),
            "fatal" | "critical" => Some(LogLevel::Fatal),
            _ => None,
        }
    }

    /// Short display string.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A structured field value.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    /// String value.
    String(String),
    /// Integer value.
    Int(i64),
    /// Float value.
    Float(f64),
    /// Boolean value.
    Bool(bool),
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldValue::String(s) => write!(f, "{s}"),
            FieldValue::Int(i) => write!(f, "{i}"),
            FieldValue::Float(v) => write!(f, "{v}"),
            FieldValue::Bool(b) => write!(f, "{b}"),
        }
    }
}

/// A structured log entry.
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Timestamp in milliseconds since epoch.
    pub timestamp_ms: u64,
    /// Log level.
    pub level: LogLevel,
    /// Log message.
    pub message: String,
    /// Source module or component.
    pub source: String,
    /// Structured fields.
    pub fields: HashMap<String, FieldValue>,
}

impl LogEntry {
    /// Create a new log entry.
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            timestamp_ms: 0,
            level,
            message: message.into(),
            source: String::new(),
            fields: HashMap::new(),
        }
    }

    /// Set the timestamp.
    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp_ms = ts;
        self
    }

    /// Set the source.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Add a string field.
    pub fn with_str(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.fields
            .insert(key.into(), FieldValue::String(val.into()));
        self
    }

    /// Add an integer field.
    pub fn with_int(mut self, key: impl Into<String>, val: i64) -> Self {
        self.fields.insert(key.into(), FieldValue::Int(val));
        self
    }

    /// Add a float field.
    pub fn with_float(mut self, key: impl Into<String>, val: f64) -> Self {
        self.fields.insert(key.into(), FieldValue::Float(val));
        self
    }

    /// Add a boolean field.
    pub fn with_bool(mut self, key: impl Into<String>, val: bool) -> Self {
        self.fields.insert(key.into(), FieldValue::Bool(val));
        self
    }

    /// Format as a single-line log string.
    pub fn format_line(&self) -> String {
        let mut parts = vec![format!("[{}] {}", self.level, self.message)];
        if !self.source.is_empty() {
            parts.push(format!("source={}", self.source));
        }
        for (k, v) in &self.fields {
            parts.push(format!("{k}={v}"));
        }
        parts.join(" ")
    }

    /// Format as JSON.
    pub fn format_json(&self) -> String {
        let mut pairs = vec![
            format!("\"level\":\"{}\"", self.level),
            format!("\"message\":\"{}\"", escape_json(&self.message)),
            format!("\"timestamp\":{}", self.timestamp_ms),
        ];
        if !self.source.is_empty() {
            pairs.push(format!("\"source\":\"{}\"", escape_json(&self.source)));
        }
        for (k, v) in &self.fields {
            let val_str = match v {
                FieldValue::String(s) => format!("\"{}\"", escape_json(s)),
                FieldValue::Int(i) => format!("{i}"),
                FieldValue::Float(f) => format!("{f}"),
                FieldValue::Bool(b) => format!("{b}"),
            };
            pairs.push(format!("\"{}\":{}", escape_json(k), val_str));
        }
        format!("{{{}}}", pairs.join(","))
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Log buffer — stores recent entries in memory.
pub struct LogBuffer {
    entries: Vec<LogEntry>,
    max_size: usize,
}

impl LogBuffer {
    /// Create a buffer with a maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
        }
    }

    /// Push an entry. If buffer is full, oldest entry is removed.
    pub fn push(&mut self, entry: LogEntry) {
        if self.entries.len() >= self.max_size {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }

    /// Get all entries.
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    /// Get entries at or above a given level.
    pub fn entries_at_level(&self, min_level: LogLevel) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level >= min_level)
            .collect()
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Count entries by level.
    pub fn count_by_level(&self) -> HashMap<LogLevel, usize> {
        let mut counts = HashMap::new();
        for e in &self.entries {
            *counts.entry(e.level).or_insert(0) += 1;
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_levels_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Fatal);
    }

    #[test]
    fn test_level_from_str() {
        assert_eq!(LogLevel::from_str_ci("INFO"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str_ci("warning"), Some(LogLevel::Warn));
        assert_eq!(LogLevel::from_str_ci("err"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str_ci("critical"), Some(LogLevel::Fatal));
        assert_eq!(LogLevel::from_str_ci("unknown"), None);
    }

    #[test]
    fn test_entry_builder() {
        let entry = LogEntry::new(LogLevel::Info, "request started")
            .with_timestamp(1000)
            .with_source("api")
            .with_str("method", "GET")
            .with_int("status", 200)
            .with_float("duration_ms", 12.5)
            .with_bool("cached", true);

        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "request started");
        assert_eq!(entry.source, "api");
        assert_eq!(entry.fields.len(), 4);
        assert_eq!(
            entry.fields.get("method"),
            Some(&FieldValue::String("GET".into()))
        );
        assert_eq!(entry.fields.get("status"), Some(&FieldValue::Int(200)));
    }

    #[test]
    fn test_format_line() {
        let entry = LogEntry::new(LogLevel::Error, "connection failed").with_source("db");
        let line = entry.format_line();
        assert!(line.contains("[ERROR]"));
        assert!(line.contains("connection failed"));
        assert!(line.contains("source=db"));
    }

    #[test]
    fn test_format_json() {
        let entry = LogEntry::new(LogLevel::Info, "test")
            .with_timestamp(42)
            .with_int("count", 5);
        let json = entry.format_json();
        assert!(json.contains("\"level\":\"INFO\""));
        assert!(json.contains("\"message\":\"test\""));
        assert!(json.contains("\"timestamp\":42"));
        assert!(json.contains("\"count\":5"));
    }

    #[test]
    fn test_json_escaping() {
        let entry = LogEntry::new(LogLevel::Info, "line1\nline2").with_str("data", "say \"hello\"");
        let json = entry.format_json();
        assert!(json.contains("\\n"));
        assert!(json.contains("\\\"hello\\\""));
    }

    #[test]
    fn test_buffer_push_and_eviction() {
        let mut buf = LogBuffer::new(3);
        buf.push(LogEntry::new(LogLevel::Info, "a"));
        buf.push(LogEntry::new(LogLevel::Warn, "b"));
        buf.push(LogEntry::new(LogLevel::Error, "c"));
        assert_eq!(buf.len(), 3);

        buf.push(LogEntry::new(LogLevel::Fatal, "d"));
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.entries()[0].message, "b"); // "a" evicted
    }

    #[test]
    fn test_buffer_level_filter() {
        let mut buf = LogBuffer::new(10);
        buf.push(LogEntry::new(LogLevel::Debug, "debug"));
        buf.push(LogEntry::new(LogLevel::Info, "info"));
        buf.push(LogEntry::new(LogLevel::Error, "error"));

        let errors = buf.entries_at_level(LogLevel::Error);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "error");
    }

    #[test]
    fn test_buffer_count_by_level() {
        let mut buf = LogBuffer::new(10);
        buf.push(LogEntry::new(LogLevel::Info, "a"));
        buf.push(LogEntry::new(LogLevel::Info, "b"));
        buf.push(LogEntry::new(LogLevel::Error, "c"));

        let counts = buf.count_by_level();
        assert_eq!(counts.get(&LogLevel::Info), Some(&2));
        assert_eq!(counts.get(&LogLevel::Error), Some(&1));
    }

    #[test]
    fn test_severity() {
        assert_eq!(LogLevel::Trace.severity(), 0);
        assert_eq!(LogLevel::Fatal.severity(), 5);
    }
}
