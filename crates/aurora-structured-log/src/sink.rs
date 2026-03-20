//! Log sinks — destinations for log entries (memory, formatted output).

use crate::entry::{LogEntry, LogLevel};
use std::collections::VecDeque;

/// Format for log output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Single-line text format.
    Text,
    /// JSON format.
    Json,
    /// Compact format (level + message only).
    Compact,
}

/// Format a log entry according to the specified format.
pub fn format_entry(entry: &LogEntry, format: LogFormat) -> String {
    match format {
        LogFormat::Text => entry.format_line(),
        LogFormat::Json => entry.format_json(),
        LogFormat::Compact => format!("{} {}", entry.level.as_str(), entry.message),
    }
}

/// In-memory log sink with configurable format and capacity.
pub struct MemorySink {
    lines: VecDeque<String>,
    max_lines: usize,
    format: LogFormat,
    min_level: LogLevel,
    total_received: u64,
    total_dropped: u64,
}

impl MemorySink {
    /// Create a new memory sink.
    pub fn new(max_lines: usize, format: LogFormat) -> Self {
        Self {
            lines: VecDeque::new(),
            max_lines,
            format,
            min_level: LogLevel::Trace,
            total_received: 0,
            total_dropped: 0,
        }
    }

    /// Set the minimum log level.
    pub fn with_min_level(mut self, level: LogLevel) -> Self {
        self.min_level = level;
        self
    }

    /// Write an entry to the sink.
    pub fn write(&mut self, entry: &LogEntry) {
        self.total_received += 1;
        if entry.level < self.min_level {
            self.total_dropped += 1;
            return;
        }
        let line = format_entry(entry, self.format);
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    /// Get all stored lines.
    pub fn lines(&self) -> Vec<&str> {
        self.lines.iter().map(|s| s.as_str()).collect()
    }

    /// Number of stored lines.
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Whether the sink is empty.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Clear all stored lines.
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Total entries received.
    pub fn total_received(&self) -> u64 {
        self.total_received
    }

    /// Total entries dropped (below min level).
    pub fn total_dropped(&self) -> u64 {
        self.total_dropped
    }
}

/// Multi-sink dispatcher — writes to multiple sinks.
pub struct MultiSink {
    sinks: Vec<MemorySink>,
}

impl MultiSink {
    /// Create a new multi-sink.
    pub fn new() -> Self {
        Self { sinks: Vec::new() }
    }

    /// Add a sink.
    pub fn add_sink(&mut self, sink: MemorySink) {
        self.sinks.push(sink);
    }

    /// Write to all sinks.
    pub fn write(&mut self, entry: &LogEntry) {
        for sink in &mut self.sinks {
            sink.write(entry);
        }
    }

    /// Number of sinks.
    pub fn sink_count(&self) -> usize {
        self.sinks.len()
    }

    /// Get a reference to a sink by index.
    pub fn get_sink(&self, index: usize) -> Option<&MemorySink> {
        self.sinks.get(index)
    }
}

impl Default for MultiSink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::LogEntry;

    #[test]
    fn test_format_text() {
        let entry = LogEntry::new(LogLevel::Info, "hello");
        let line = format_entry(&entry, LogFormat::Text);
        assert!(line.contains("[INFO]"));
        assert!(line.contains("hello"));
    }

    #[test]
    fn test_format_json() {
        let entry = LogEntry::new(LogLevel::Warn, "oops").with_timestamp(100);
        let json = format_entry(&entry, LogFormat::Json);
        assert!(json.starts_with('{'));
        assert!(json.contains("\"level\":\"WARN\""));
    }

    #[test]
    fn test_format_compact() {
        let entry = LogEntry::new(LogLevel::Error, "fail");
        let line = format_entry(&entry, LogFormat::Compact);
        assert_eq!(line, "ERROR fail");
    }

    #[test]
    fn test_memory_sink() {
        let mut sink = MemorySink::new(5, LogFormat::Compact);
        sink.write(&LogEntry::new(LogLevel::Info, "a"));
        sink.write(&LogEntry::new(LogLevel::Warn, "b"));
        assert_eq!(sink.len(), 2);
        assert_eq!(sink.lines()[0], "INFO a");
        assert_eq!(sink.lines()[1], "WARN b");
    }

    #[test]
    fn test_memory_sink_eviction() {
        let mut sink = MemorySink::new(2, LogFormat::Compact);
        sink.write(&LogEntry::new(LogLevel::Info, "a"));
        sink.write(&LogEntry::new(LogLevel::Info, "b"));
        sink.write(&LogEntry::new(LogLevel::Info, "c"));
        assert_eq!(sink.len(), 2);
        assert_eq!(sink.lines()[0], "INFO b");
    }

    #[test]
    fn test_memory_sink_min_level() {
        let mut sink = MemorySink::new(10, LogFormat::Compact).with_min_level(LogLevel::Warn);
        sink.write(&LogEntry::new(LogLevel::Debug, "debug"));
        sink.write(&LogEntry::new(LogLevel::Warn, "warn"));
        sink.write(&LogEntry::new(LogLevel::Error, "error"));

        assert_eq!(sink.len(), 2);
        assert_eq!(sink.total_received(), 3);
        assert_eq!(sink.total_dropped(), 1);
    }

    #[test]
    fn test_multi_sink() {
        let mut multi = MultiSink::new();
        multi.add_sink(MemorySink::new(10, LogFormat::Compact));
        multi.add_sink(MemorySink::new(10, LogFormat::Json));

        multi.write(&LogEntry::new(LogLevel::Info, "test"));

        assert_eq!(multi.sink_count(), 2);
        assert_eq!(multi.get_sink(0).unwrap().len(), 1);
        assert_eq!(multi.get_sink(1).unwrap().len(), 1);
    }

    #[test]
    fn test_sink_clear() {
        let mut sink = MemorySink::new(10, LogFormat::Compact);
        sink.write(&LogEntry::new(LogLevel::Info, "a"));
        sink.clear();
        assert!(sink.is_empty());
    }
}
