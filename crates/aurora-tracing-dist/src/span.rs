//! Span management — create, annotate, and close spans within a trace.

use crate::context::{SpanId, TraceId};
use std::collections::HashMap;

/// Span status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanStatus {
    /// Span is currently active.
    Active,
    /// Span completed successfully.
    Ok,
    /// Span completed with an error.
    Error,
}

/// A span event (annotation at a point in time).
#[derive(Debug, Clone)]
pub struct SpanEvent {
    /// Timestamp in microseconds since epoch.
    pub timestamp_us: u64,
    /// Event name.
    pub name: String,
    /// Event attributes.
    pub attributes: HashMap<String, String>,
}

impl SpanEvent {
    /// Create a new span event.
    pub fn new(name: impl Into<String>, timestamp_us: u64) -> Self {
        Self {
            timestamp_us,
            name: name.into(),
            attributes: HashMap::new(),
        }
    }

    /// Add an attribute.
    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// A trace span.
#[derive(Debug, Clone)]
pub struct Span {
    /// Trace this span belongs to.
    pub trace_id: TraceId,
    /// This span's ID.
    pub span_id: SpanId,
    /// Parent span ID (None if root).
    pub parent_id: Option<SpanId>,
    /// Operation name.
    pub name: String,
    /// Service name.
    pub service: String,
    /// Start timestamp in microseconds.
    pub start_us: u64,
    /// End timestamp in microseconds (0 if still active).
    pub end_us: u64,
    /// Span status.
    pub status: SpanStatus,
    /// Attributes.
    pub attributes: HashMap<String, String>,
    /// Events within the span.
    pub events: Vec<SpanEvent>,
}

impl Span {
    /// Create a new root span.
    pub fn new_root(
        trace_id: TraceId,
        span_id: SpanId,
        name: impl Into<String>,
        start_us: u64,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            parent_id: None,
            name: name.into(),
            service: String::new(),
            start_us,
            end_us: 0,
            status: SpanStatus::Active,
            attributes: HashMap::new(),
            events: Vec::new(),
        }
    }

    /// Create a child span.
    pub fn new_child(
        trace_id: TraceId,
        span_id: SpanId,
        parent_id: SpanId,
        name: impl Into<String>,
        start_us: u64,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            parent_id: Some(parent_id),
            name: name.into(),
            service: String::new(),
            start_us,
            end_us: 0,
            status: SpanStatus::Active,
            attributes: HashMap::new(),
            events: Vec::new(),
        }
    }

    /// Set the service name.
    pub fn with_service(mut self, service: impl Into<String>) -> Self {
        self.service = service.into();
        self
    }

    /// Set an attribute.
    pub fn set_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(key.into(), value.into());
    }

    /// Add an event.
    pub fn add_event(&mut self, event: SpanEvent) {
        self.events.push(event);
    }

    /// End the span successfully.
    pub fn end_ok(&mut self, end_us: u64) {
        self.end_us = end_us;
        self.status = SpanStatus::Ok;
    }

    /// End the span with an error.
    pub fn end_error(&mut self, end_us: u64, error_msg: impl Into<String>) {
        self.end_us = end_us;
        self.status = SpanStatus::Error;
        self.attributes.insert("error".into(), error_msg.into());
    }

    /// Duration in microseconds (0 if still active).
    pub fn duration_us(&self) -> u64 {
        if self.end_us > self.start_us {
            self.end_us - self.start_us
        } else {
            0
        }
    }

    /// Whether this span is a root span.
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Whether this span is still active.
    pub fn is_active(&self) -> bool {
        self.status == SpanStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tid() -> TraceId {
        TraceId::new(1, 1)
    }

    #[test]
    fn test_root_span() {
        let span = Span::new_root(tid(), SpanId::new(100), "request", 1000);
        assert!(span.is_root());
        assert!(span.is_active());
        assert_eq!(span.name, "request");
        assert_eq!(span.start_us, 1000);
    }

    #[test]
    fn test_child_span() {
        let span = Span::new_child(tid(), SpanId::new(200), SpanId::new(100), "db.query", 1500);
        assert!(!span.is_root());
        assert_eq!(span.parent_id, Some(SpanId::new(100)));
    }

    #[test]
    fn test_span_end_ok() {
        let mut span = Span::new_root(tid(), SpanId::new(1), "op", 1000);
        span.end_ok(2000);
        assert_eq!(span.status, SpanStatus::Ok);
        assert_eq!(span.duration_us(), 1000);
        assert!(!span.is_active());
    }

    #[test]
    fn test_span_end_error() {
        let mut span = Span::new_root(tid(), SpanId::new(1), "op", 1000);
        span.end_error(2000, "timeout");
        assert_eq!(span.status, SpanStatus::Error);
        assert_eq!(span.attributes.get("error"), Some(&"timeout".to_string()));
    }

    #[test]
    fn test_span_attributes() {
        let mut span = Span::new_root(tid(), SpanId::new(1), "op", 0).with_service("gateway");
        span.set_attr("http.method", "GET");
        span.set_attr("http.status", "200");

        assert_eq!(span.service, "gateway");
        assert_eq!(span.attributes.get("http.method"), Some(&"GET".to_string()));
    }

    #[test]
    fn test_span_events() {
        let mut span = Span::new_root(tid(), SpanId::new(1), "op", 0);
        span.add_event(SpanEvent::new("cache.hit", 500).with_attr("key", "user:42"));

        assert_eq!(span.events.len(), 1);
        assert_eq!(span.events[0].name, "cache.hit");
        assert_eq!(
            span.events[0].attributes.get("key"),
            Some(&"user:42".to_string())
        );
    }

    #[test]
    fn test_active_duration() {
        let span = Span::new_root(tid(), SpanId::new(1), "op", 1000);
        assert_eq!(span.duration_us(), 0); // still active
    }
}
