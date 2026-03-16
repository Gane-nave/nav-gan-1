//! Trace collector — aggregates spans into complete traces.

use crate::context::{SpanId, TraceId};
use crate::span::{Span, SpanStatus};
use std::collections::HashMap;

/// A complete trace composed of multiple spans.
#[derive(Debug, Clone)]
pub struct Trace {
    /// Trace ID.
    pub trace_id: TraceId,
    /// All spans in this trace.
    pub spans: Vec<Span>,
}

impl Trace {
    /// Create a new trace.
    pub fn new(trace_id: TraceId) -> Self {
        Self {
            trace_id,
            spans: Vec::new(),
        }
    }

    /// Add a span.
    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    /// Find the root span.
    pub fn root_span(&self) -> Option<&Span> {
        self.spans.iter().find(|s| s.is_root())
    }

    /// Find children of a given span.
    pub fn children_of(&self, span_id: SpanId) -> Vec<&Span> {
        self.spans
            .iter()
            .filter(|s| s.parent_id == Some(span_id))
            .collect()
    }

    /// Total duration (root span duration).
    pub fn duration_us(&self) -> u64 {
        self.root_span().map_or(0, |s| s.duration_us())
    }

    /// Number of spans.
    pub fn span_count(&self) -> usize {
        self.spans.len()
    }

    /// Depth of the trace (longest chain from root).
    pub fn depth(&self) -> usize {
        let root = match self.root_span() {
            Some(r) => r.span_id,
            None => return 0,
        };
        self.depth_from(root)
    }

    fn depth_from(&self, span_id: SpanId) -> usize {
        let children = self.children_of(span_id);
        if children.is_empty() {
            return 1;
        }
        1 + children
            .iter()
            .map(|c| self.depth_from(c.span_id))
            .max()
            .unwrap_or(0)
    }

    /// Check if trace has any errors.
    pub fn has_errors(&self) -> bool {
        self.spans.iter().any(|s| s.status == SpanStatus::Error)
    }

    /// Get all error spans.
    pub fn error_spans(&self) -> Vec<&Span> {
        self.spans
            .iter()
            .filter(|s| s.status == SpanStatus::Error)
            .collect()
    }

    /// Get all services involved in this trace.
    pub fn services(&self) -> Vec<&str> {
        let mut svcs: Vec<&str> = self
            .spans
            .iter()
            .filter(|s| !s.service.is_empty())
            .map(|s| s.service.as_str())
            .collect();
        svcs.sort();
        svcs.dedup();
        svcs
    }
}

/// Trace collector — collects spans and assembles them into traces.
pub struct TraceCollector {
    traces: HashMap<TraceId, Trace>,
    max_traces: usize,
    completed_count: u64,
}

impl TraceCollector {
    /// Create a new collector with a maximum trace limit.
    pub fn new(max_traces: usize) -> Self {
        Self {
            traces: HashMap::new(),
            max_traces,
            completed_count: 0,
        }
    }

    /// Record a span.
    pub fn record_span(&mut self, span: Span) {
        let trace = self
            .traces
            .entry(span.trace_id)
            .or_insert_with(|| Trace::new(span.trace_id));
        trace.add_span(span);

        // Evict oldest traces if over limit
        if self.traces.len() > self.max_traces {
            // Remove trace with smallest trace_id.1 as a simple eviction
            if let Some(&key) = self.traces.keys().min_by_key(|k| k.1) {
                self.traces.remove(&key);
                self.completed_count += 1;
            }
        }
    }

    /// Get a trace by ID.
    pub fn get_trace(&self, trace_id: &TraceId) -> Option<&Trace> {
        self.traces.get(trace_id)
    }

    /// Number of active traces.
    pub fn active_traces(&self) -> usize {
        self.traces.len()
    }

    /// Total completed (evicted) traces.
    pub fn completed_count(&self) -> u64 {
        self.completed_count
    }

    /// Remove a trace.
    pub fn remove_trace(&mut self, trace_id: &TraceId) -> Option<Trace> {
        let trace = self.traces.remove(trace_id);
        if trace.is_some() {
            self.completed_count += 1;
        }
        trace
    }

    /// Find traces with errors.
    pub fn traces_with_errors(&self) -> Vec<&Trace> {
        self.traces.values().filter(|t| t.has_errors()).collect()
    }

    /// Clear all traces.
    pub fn clear(&mut self) {
        self.completed_count += self.traces.len() as u64;
        self.traces.clear();
    }

    /// Get summary statistics.
    pub fn stats(&self) -> CollectorStats {
        let total_spans: usize = self.traces.values().map(|t| t.span_count()).sum();
        let error_traces = self.traces.values().filter(|t| t.has_errors()).count();

        CollectorStats {
            active_traces: self.traces.len(),
            completed_traces: self.completed_count,
            total_spans,
            error_traces,
        }
    }
}

/// Collector statistics.
#[derive(Debug, Clone)]
pub struct CollectorStats {
    pub active_traces: usize,
    pub completed_traces: u64,
    pub total_spans: usize,
    pub error_traces: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::SpanId;
    use crate::span::Span;

    fn tid(n: u64) -> TraceId {
        TraceId::new(0, n)
    }

    fn make_root(trace_n: u64, span_n: u64, name: &str) -> Span {
        Span::new_root(tid(trace_n), SpanId::new(span_n), name, 0).with_service("test-svc")
    }

    fn make_child(trace_n: u64, span_n: u64, parent_n: u64, name: &str) -> Span {
        Span::new_child(
            tid(trace_n),
            SpanId::new(span_n),
            SpanId::new(parent_n),
            name,
            100,
        )
        .with_service("test-svc")
    }

    #[test]
    fn test_trace_assembly() {
        let mut trace = Trace::new(tid(1));
        let mut root = make_root(1, 1, "request");
        root.end_ok(1000);
        trace.add_span(root);
        trace.add_span(make_child(1, 2, 1, "db.query"));

        assert_eq!(trace.span_count(), 2);
        assert!(trace.root_span().is_some());
        assert_eq!(trace.children_of(SpanId::new(1)).len(), 1);
        assert_eq!(trace.duration_us(), 1000);
    }

    #[test]
    fn test_trace_depth() {
        let mut trace = Trace::new(tid(1));
        trace.add_span(make_root(1, 1, "root"));
        trace.add_span(make_child(1, 2, 1, "child"));
        trace.add_span(make_child(1, 3, 2, "grandchild"));

        assert_eq!(trace.depth(), 3);
    }

    #[test]
    fn test_trace_errors() {
        let mut trace = Trace::new(tid(1));
        trace.add_span(make_root(1, 1, "root"));
        let mut err_span = make_child(1, 2, 1, "fail");
        err_span.end_error(200, "timeout");
        trace.add_span(err_span);

        assert!(trace.has_errors());
        assert_eq!(trace.error_spans().len(), 1);
    }

    #[test]
    fn test_trace_services() {
        let mut trace = Trace::new(tid(1));
        let mut s1 = make_root(1, 1, "root");
        s1.service = "gateway".into();
        let mut s2 = make_child(1, 2, 1, "query");
        s2.service = "db".into();
        trace.add_span(s1);
        trace.add_span(s2);

        let services = trace.services();
        assert_eq!(services.len(), 2);
        assert!(services.contains(&"gateway"));
        assert!(services.contains(&"db"));
    }

    #[test]
    fn test_collector_record() {
        let mut collector = TraceCollector::new(100);
        collector.record_span(make_root(1, 1, "req1"));
        collector.record_span(make_root(2, 2, "req2"));

        assert_eq!(collector.active_traces(), 2);
        assert!(collector.get_trace(&tid(1)).is_some());
    }

    #[test]
    fn test_collector_eviction() {
        let mut collector = TraceCollector::new(2);
        collector.record_span(make_root(1, 1, "a"));
        collector.record_span(make_root(2, 2, "b"));
        collector.record_span(make_root(3, 3, "c")); // evicts one

        assert_eq!(collector.active_traces(), 2);
        assert_eq!(collector.completed_count(), 1);
    }

    #[test]
    fn test_collector_remove() {
        let mut collector = TraceCollector::new(100);
        collector.record_span(make_root(1, 1, "req"));
        let trace = collector.remove_trace(&tid(1));
        assert!(trace.is_some());
        assert_eq!(collector.active_traces(), 0);
        assert_eq!(collector.completed_count(), 1);
    }

    #[test]
    fn test_collector_error_traces() {
        let mut collector = TraceCollector::new(100);
        collector.record_span(make_root(1, 1, "ok"));
        let mut err = make_root(2, 2, "fail");
        err.end_error(100, "crash");
        collector.record_span(err);

        assert_eq!(collector.traces_with_errors().len(), 1);
    }

    #[test]
    fn test_collector_stats() {
        let mut collector = TraceCollector::new(100);
        collector.record_span(make_root(1, 1, "a"));
        collector.record_span(make_child(1, 2, 1, "b"));

        let stats = collector.stats();
        assert_eq!(stats.active_traces, 1);
        assert_eq!(stats.total_spans, 2);
    }

    #[test]
    fn test_collector_clear() {
        let mut collector = TraceCollector::new(100);
        collector.record_span(make_root(1, 1, "a"));
        collector.record_span(make_root(2, 2, "b"));
        collector.clear();
        assert_eq!(collector.active_traces(), 0);
        assert_eq!(collector.completed_count(), 2);
    }
}
