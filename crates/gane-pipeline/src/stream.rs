//! Stream processing — real-time data windowing, aggregation, and watermarks.

use std::collections::VecDeque;

/// Window type for stream processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    /// Tumbling window — fixed, non-overlapping.
    Tumbling,
    /// Sliding window — overlapping with a slide interval.
    Sliding,
    /// Session window — gap-based.
    Session,
}

/// Aggregation function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateFunc {
    Sum,
    Count,
    Min,
    Max,
    Avg,
}

/// A single stream event.
#[derive(Debug, Clone)]
pub struct StreamEvent {
    /// Event timestamp (epoch millis).
    pub timestamp_ms: u64,
    /// Event value (numeric).
    pub value: f64,
    /// Event key (for grouping).
    pub key: String,
}

/// Stream window — accumulates events within a time range.
pub struct StreamWindow {
    window_type: WindowType,
    /// Window duration in milliseconds.
    duration_ms: u64,
    /// Events in the current window.
    events: VecDeque<StreamEvent>,
    /// Window start time.
    window_start_ms: u64,
    /// Total windows completed.
    windows_completed: u64,
}

impl StreamWindow {
    /// Create a new stream window.
    pub fn new(window_type: WindowType, duration_ms: u64, start_ms: u64) -> Self {
        Self {
            window_type,
            duration_ms,
            events: VecDeque::new(),
            window_start_ms: start_ms,
            windows_completed: 0,
        }
    }

    /// Add an event to the window.
    /// Returns true if the event falls within the current window.
    pub fn add_event(&mut self, event: StreamEvent) -> bool {
        if event.timestamp_ms < self.window_start_ms {
            return false; // late event, before window start
        }

        match self.window_type {
            WindowType::Tumbling | WindowType::Sliding => {
                if event.timestamp_ms >= self.window_start_ms + self.duration_ms {
                    return false; // beyond current window
                }
                self.events.push_back(event);
                true
            }
            WindowType::Session => {
                // Session windows: keep event if within gap duration of last event
                if let Some(last) = self.events.back() {
                    if event.timestamp_ms > last.timestamp_ms + self.duration_ms {
                        return false; // gap exceeded
                    }
                }
                self.events.push_back(event);
                true
            }
        }
    }

    /// Advance the window — flush current window and start a new one.
    pub fn advance(&mut self) -> Vec<StreamEvent> {
        self.windows_completed += 1;
        self.window_start_ms += self.duration_ms;
        let old_events: Vec<StreamEvent> = self.events.drain(..).collect();
        old_events
    }

    /// Compute an aggregate over the current window events.
    pub fn aggregate(&self, func: AggregateFunc) -> f64 {
        if self.events.is_empty() {
            return 0.0;
        }

        match func {
            AggregateFunc::Sum => self.events.iter().map(|e| e.value).sum(),
            AggregateFunc::Count => self.events.len() as f64,
            AggregateFunc::Min => self
                .events
                .iter()
                .map(|e| e.value)
                .fold(f64::INFINITY, f64::min),
            AggregateFunc::Max => self
                .events
                .iter()
                .map(|e| e.value)
                .fold(f64::NEG_INFINITY, f64::max),
            AggregateFunc::Avg => {
                let sum: f64 = self.events.iter().map(|e| e.value).sum();
                sum / self.events.len() as f64
            }
        }
    }

    /// Get the number of events in the current window.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Get total windows completed.
    pub fn windows_completed(&self) -> u64 {
        self.windows_completed
    }

    /// Get window type.
    pub fn window_type(&self) -> WindowType {
        self.window_type
    }

    /// Get window start time.
    pub fn window_start(&self) -> u64 {
        self.window_start_ms
    }

    /// Get window end time.
    pub fn window_end(&self) -> u64 {
        self.window_start_ms + self.duration_ms
    }
}

/// Watermark tracker — tracks event-time progress for out-of-order handling.
pub struct WatermarkTracker {
    /// Current watermark (epoch millis).
    watermark_ms: u64,
    /// Maximum allowed lateness (millis).
    max_lateness_ms: u64,
    /// Count of late events dropped.
    late_events_dropped: u64,
    /// Count of events accepted.
    events_accepted: u64,
}

impl WatermarkTracker {
    /// Create a new watermark tracker.
    pub fn new(max_lateness_ms: u64) -> Self {
        Self {
            watermark_ms: 0,
            max_lateness_ms,
            late_events_dropped: 0,
            events_accepted: 0,
        }
    }

    /// Process an event — returns true if accepted, false if too late.
    pub fn process(&mut self, event_timestamp_ms: u64) -> bool {
        if event_timestamp_ms + self.max_lateness_ms < self.watermark_ms {
            self.late_events_dropped += 1;
            return false;
        }
        self.events_accepted += 1;
        if event_timestamp_ms > self.watermark_ms {
            self.watermark_ms = event_timestamp_ms;
        }
        true
    }

    /// Get current watermark.
    pub fn watermark(&self) -> u64 {
        self.watermark_ms
    }

    /// Get count of late events dropped.
    pub fn late_events_dropped(&self) -> u64 {
        self.late_events_dropped
    }

    /// Get count of accepted events.
    pub fn events_accepted(&self) -> u64 {
        self.events_accepted
    }

    /// Get late event rate.
    pub fn late_rate(&self) -> f64 {
        let total = self.events_accepted + self.late_events_dropped;
        if total == 0 {
            return 0.0;
        }
        self.late_events_dropped as f64 / total as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(ts: u64, value: f64) -> StreamEvent {
        StreamEvent {
            timestamp_ms: ts,
            value,
            key: "default".to_string(),
        }
    }

    #[test]
    fn test_tumbling_window_accepts_in_range() {
        let mut w = StreamWindow::new(WindowType::Tumbling, 1000, 0);
        assert!(w.add_event(make_event(0, 1.0)));
        assert!(w.add_event(make_event(500, 2.0)));
        assert!(w.add_event(make_event(999, 3.0)));
        assert!(!w.add_event(make_event(1000, 4.0))); // outside window
        assert_eq!(w.event_count(), 3);
    }

    #[test]
    fn test_tumbling_window_rejects_late() {
        let mut w = StreamWindow::new(WindowType::Tumbling, 1000, 100);
        assert!(!w.add_event(make_event(50, 1.0))); // before window start
    }

    #[test]
    fn test_window_aggregate_sum() {
        let mut w = StreamWindow::new(WindowType::Tumbling, 1000, 0);
        w.add_event(make_event(0, 10.0));
        w.add_event(make_event(100, 20.0));
        w.add_event(make_event(200, 30.0));
        assert!((w.aggregate(AggregateFunc::Sum) - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_window_aggregate_avg() {
        let mut w = StreamWindow::new(WindowType::Tumbling, 1000, 0);
        w.add_event(make_event(0, 10.0));
        w.add_event(make_event(100, 20.0));
        w.add_event(make_event(200, 30.0));
        assert!((w.aggregate(AggregateFunc::Avg) - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_window_aggregate_min_max() {
        let mut w = StreamWindow::new(WindowType::Tumbling, 1000, 0);
        w.add_event(make_event(0, 5.0));
        w.add_event(make_event(100, 15.0));
        w.add_event(make_event(200, 10.0));
        assert!((w.aggregate(AggregateFunc::Min) - 5.0).abs() < f64::EPSILON);
        assert!((w.aggregate(AggregateFunc::Max) - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_window_advance() {
        let mut w = StreamWindow::new(WindowType::Tumbling, 1000, 0);
        w.add_event(make_event(0, 1.0));
        w.add_event(make_event(500, 2.0));
        let flushed = w.advance();
        assert_eq!(flushed.len(), 2);
        assert_eq!(w.event_count(), 0);
        assert_eq!(w.windows_completed(), 1);
        assert_eq!(w.window_start(), 1000);
    }

    #[test]
    fn test_session_window_gap() {
        let mut w = StreamWindow::new(WindowType::Session, 500, 0);
        assert!(w.add_event(make_event(0, 1.0)));
        assert!(w.add_event(make_event(300, 2.0))); // within gap
        assert!(w.add_event(make_event(700, 3.0))); // within gap of last (300+500=800)
        assert!(!w.add_event(make_event(1300, 4.0))); // gap exceeded (700+500=1200 < 1300)
        assert_eq!(w.event_count(), 3);
    }

    #[test]
    fn test_watermark_accepts_in_order() {
        let mut wm = WatermarkTracker::new(100);
        assert!(wm.process(1000));
        assert!(wm.process(2000));
        assert!(wm.process(3000));
        assert_eq!(wm.watermark(), 3000);
        assert_eq!(wm.events_accepted(), 3);
    }

    #[test]
    fn test_watermark_drops_too_late() {
        let mut wm = WatermarkTracker::new(100);
        wm.process(1000);
        wm.process(2000);
        // Event at 1800 — lateness = 2000 - 1800 = 200 > max_lateness(100)
        assert!(!wm.process(1800));
        assert_eq!(wm.late_events_dropped(), 1);
    }

    #[test]
    fn test_watermark_accepts_slightly_late() {
        let mut wm = WatermarkTracker::new(100);
        wm.process(1000);
        wm.process(2000);
        // Event at 1950 — lateness = 2000 - 1950 = 50 <= max_lateness(100)
        assert!(wm.process(1950));
    }

    #[test]
    fn test_watermark_late_rate() {
        let mut wm = WatermarkTracker::new(50);
        wm.process(1000);
        wm.process(2000);
        wm.process(500); // too late: 500 + 50 = 550 < 2000
        wm.process(3000);
        // 3 accepted, 1 dropped → late_rate = 1/4 = 0.25
        assert!((wm.late_rate() - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn test_empty_window_aggregate() {
        let w = StreamWindow::new(WindowType::Tumbling, 1000, 0);
        assert!((w.aggregate(AggregateFunc::Sum) - 0.0).abs() < f64::EPSILON);
        assert!((w.aggregate(AggregateFunc::Count) - 0.0).abs() < f64::EPSILON);
    }
}
