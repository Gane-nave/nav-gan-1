//! Profiling — function timing, span tracking, performance metrics collection.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A profiling span — measures elapsed time for an operation.
#[derive(Debug, Clone)]
pub struct Span {
    pub name: String,
    pub category: String,
    start: Instant,
    end: Option<Instant>,
    pub metadata: HashMap<String, String>,
}

impl Span {
    /// Start a new profiling span.
    pub fn start(name: &str, category: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            start: Instant::now(),
            end: None,
            metadata: HashMap::new(),
        }
    }

    /// End the span.
    pub fn end(&mut self) {
        self.end = Some(Instant::now());
    }

    /// Get elapsed duration.
    pub fn elapsed(&self) -> Duration {
        match self.end {
            Some(end) => end.duration_since(self.start),
            None => self.start.elapsed(),
        }
    }

    /// Check if the span is still running.
    pub fn is_running(&self) -> bool {
        self.end.is_none()
    }

    /// Add metadata.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Aggregated statistics for a metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStats {
    pub name: String,
    pub count: u64,
    pub total_us: u64,
    pub min_us: u64,
    pub max_us: u64,
    pub avg_us: f64,
    pub p50_us: u64,
    pub p95_us: u64,
    pub p99_us: u64,
}

/// Performance profiler — collects spans and computes statistics.
pub struct Profiler {
    spans: RwLock<Vec<Span>>,
    timings: RwLock<HashMap<String, Vec<u64>>>,
    enabled: RwLock<bool>,
    max_samples: usize,
}

impl Profiler {
    /// Create a new profiler.
    pub fn new(max_samples: usize) -> Self {
        Self {
            spans: RwLock::new(Vec::new()),
            timings: RwLock::new(HashMap::new()),
            enabled: RwLock::new(true),
            max_samples,
        }
    }

    /// Enable or disable profiling.
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.write() = enabled;
    }

    /// Check if profiling is enabled.
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }

    /// Record a completed span.
    pub fn record_span(&self, span: &Span) {
        if !self.is_enabled() {
            return;
        }

        let elapsed_us = span.elapsed().as_micros() as u64;
        let mut timings = self.timings.write();
        let samples = timings.entry(span.name.clone()).or_default();
        if samples.len() >= self.max_samples {
            samples.remove(0);
        }
        samples.push(elapsed_us);
    }

    /// Record a timing directly.
    pub fn record_timing(&self, name: &str, duration: Duration) {
        if !self.is_enabled() {
            return;
        }

        let elapsed_us = duration.as_micros() as u64;
        let mut timings = self.timings.write();
        let samples = timings.entry(name.to_string()).or_default();
        if samples.len() >= self.max_samples {
            samples.remove(0);
        }
        samples.push(elapsed_us);
    }

    /// Get statistics for a specific metric.
    pub fn stats(&self, name: &str) -> Option<MetricStats> {
        let timings = self.timings.read();
        timings
            .get(name)
            .map(|samples| Self::compute_stats(name, samples))
    }

    /// Get statistics for all metrics.
    pub fn all_stats(&self) -> Vec<MetricStats> {
        let timings = self.timings.read();
        timings
            .iter()
            .map(|(name, samples)| Self::compute_stats(name, samples))
            .collect()
    }

    fn compute_stats(name: &str, samples: &[u64]) -> MetricStats {
        if samples.is_empty() {
            return MetricStats {
                name: name.to_string(),
                count: 0,
                total_us: 0,
                min_us: 0,
                max_us: 0,
                avg_us: 0.0,
                p50_us: 0,
                p95_us: 0,
                p99_us: 0,
            };
        }

        let mut sorted = samples.to_vec();
        sorted.sort_unstable();

        let count = sorted.len() as u64;
        let total: u64 = sorted.iter().sum();
        let min = sorted[0];
        let max = *sorted.last().unwrap();
        let avg = total as f64 / count as f64;
        let p50 = percentile(&sorted, 50.0);
        let p95 = percentile(&sorted, 95.0);
        let p99 = percentile(&sorted, 99.0);

        MetricStats {
            name: name.to_string(),
            count,
            total_us: total,
            min_us: min,
            max_us: max,
            avg_us: avg,
            p50_us: p50,
            p95_us: p95,
            p99_us: p99,
        }
    }

    /// Clear all recorded data.
    pub fn clear(&self) {
        self.spans.write().clear();
        self.timings.write().clear();
    }

    /// Number of tracked metrics.
    pub fn metric_count(&self) -> usize {
        self.timings.read().len()
    }

    /// Total number of samples across all metrics.
    pub fn total_samples(&self) -> usize {
        self.timings.read().values().map(|v| v.len()).sum()
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new(10000)
    }
}

/// Calculate percentile from sorted values.
fn percentile(sorted: &[u64], pct: f64) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let idx = ((pct / 100.0) * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Frame rate tracker — monitors render performance.
pub struct FrameRateTracker {
    frame_times: RwLock<Vec<Duration>>,
    max_frames: usize,
    target_fps: f64,
}

impl FrameRateTracker {
    /// Create a new frame rate tracker.
    pub fn new(max_frames: usize, target_fps: f64) -> Self {
        Self {
            frame_times: RwLock::new(Vec::new()),
            max_frames,
            target_fps,
        }
    }

    /// Record a frame time.
    pub fn record_frame(&self, duration: Duration) {
        let mut frames = self.frame_times.write();
        if frames.len() >= self.max_frames {
            frames.remove(0);
        }
        frames.push(duration);
    }

    /// Get the current FPS.
    pub fn fps(&self) -> f64 {
        let frames = self.frame_times.read();
        if frames.is_empty() {
            return 0.0;
        }
        let total: Duration = frames.iter().sum();
        let avg_frame_time = total.as_secs_f64() / frames.len() as f64;
        if avg_frame_time > 0.0 {
            1.0 / avg_frame_time
        } else {
            0.0
        }
    }

    /// Check if FPS is below target.
    pub fn is_below_target(&self) -> bool {
        let fps = self.fps();
        fps > 0.0 && fps < self.target_fps
    }

    /// Get the target frame time.
    pub fn target_frame_time(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.target_fps)
    }

    /// Number of recorded frames.
    pub fn frame_count(&self) -> usize {
        self.frame_times.read().len()
    }

    /// Get jank ratio (frames that exceeded 2x target frame time).
    pub fn jank_ratio(&self) -> f64 {
        let frames = self.frame_times.read();
        if frames.is_empty() {
            return 0.0;
        }
        let target = self.target_frame_time();
        let jank_threshold = target * 2;
        let janky = frames.iter().filter(|f| **f > jank_threshold).count();
        janky as f64 / frames.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_basic() {
        let mut span = Span::start("test_op", "test");
        assert!(span.is_running());
        std::thread::sleep(Duration::from_millis(1));
        span.end();
        assert!(!span.is_running());
        assert!(span.elapsed().as_micros() > 0);
    }

    #[test]
    fn test_span_metadata() {
        let span = Span::start("routing", "navigation").with_metadata("waypoints", "5");
        assert_eq!(span.metadata.get("waypoints"), Some(&"5".to_string()));
    }

    #[test]
    fn test_profiler_record() {
        let profiler = Profiler::new(100);
        profiler.record_timing("fusion", Duration::from_micros(100));
        profiler.record_timing("fusion", Duration::from_micros(200));
        profiler.record_timing("fusion", Duration::from_micros(300));

        let stats = profiler.stats("fusion").unwrap();
        assert_eq!(stats.count, 3);
        assert_eq!(stats.min_us, 100);
        assert_eq!(stats.max_us, 300);
        assert!((stats.avg_us - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_profiler_record_span() {
        let profiler = Profiler::new(100);
        let mut span = Span::start("render", "ui");
        std::thread::sleep(Duration::from_millis(1));
        span.end();
        profiler.record_span(&span);

        let stats = profiler.stats("render").unwrap();
        assert_eq!(stats.count, 1);
        assert!(stats.min_us > 0);
    }

    #[test]
    fn test_profiler_disabled() {
        let profiler = Profiler::new(100);
        profiler.set_enabled(false);
        profiler.record_timing("test", Duration::from_micros(100));
        assert!(profiler.stats("test").is_none());
    }

    #[test]
    fn test_profiler_max_samples() {
        let profiler = Profiler::new(3);
        for i in 0..5 {
            profiler.record_timing("op", Duration::from_micros(i * 100));
        }
        let stats = profiler.stats("op").unwrap();
        assert_eq!(stats.count, 3);
    }

    #[test]
    fn test_profiler_all_stats() {
        let profiler = Profiler::new(100);
        profiler.record_timing("a", Duration::from_micros(10));
        profiler.record_timing("b", Duration::from_micros(20));
        let all = profiler.all_stats();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_percentile() {
        let sorted = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // With 10 elements, p50 = index round(0.5*9)=round(4.5)=5 → value 6
        assert_eq!(percentile(&sorted, 50.0), 6);
        assert_eq!(percentile(&sorted, 0.0), 1);
        assert_eq!(percentile(&sorted, 100.0), 10);
    }

    #[test]
    fn test_percentile_empty() {
        assert_eq!(percentile(&[], 50.0), 0);
    }

    #[test]
    fn test_frame_rate_tracker() {
        let tracker = FrameRateTracker::new(100, 60.0);
        // Simulate ~65 FPS (well above 60 target) to avoid float precision issues
        for _ in 0..10 {
            tracker.record_frame(Duration::from_micros(15384)); // ~65 FPS
        }
        let fps = tracker.fps();
        assert!(fps > 60.0);
        assert!(!tracker.is_below_target());
    }

    #[test]
    fn test_frame_rate_below_target() {
        let tracker = FrameRateTracker::new(100, 60.0);
        // Simulate 30 FPS
        for _ in 0..10 {
            tracker.record_frame(Duration::from_micros(33333));
        }
        assert!(tracker.is_below_target());
    }

    #[test]
    fn test_jank_ratio() {
        let tracker = FrameRateTracker::new(100, 60.0);
        // 8 smooth frames + 2 janky frames
        for _ in 0..8 {
            tracker.record_frame(Duration::from_micros(16000));
        }
        for _ in 0..2 {
            tracker.record_frame(Duration::from_micros(50000)); // 3x target
        }
        let ratio = tracker.jank_ratio();
        assert!((ratio - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_profiler_clear() {
        let profiler = Profiler::new(100);
        profiler.record_timing("a", Duration::from_micros(10));
        profiler.clear();
        assert_eq!(profiler.metric_count(), 0);
        assert_eq!(profiler.total_samples(), 0);
    }
}
