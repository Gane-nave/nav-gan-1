//! Sliding window counter — tracks event rates over a rolling time window.

/// A sliding window counter for tracking event rates.
pub struct SlidingWindow {
    /// Window duration in milliseconds.
    window_ms: u64,
    /// Number of sub-buckets for granularity.
    bucket_count: usize,
    /// Counts per sub-bucket.
    buckets: Vec<u64>,
    /// Timestamp of each sub-bucket start.
    bucket_times: Vec<u64>,
    /// Duration of each sub-bucket.
    bucket_duration_ms: u64,
    /// Current bucket index.
    current_index: usize,
    /// Total events tracked (lifetime).
    total_events: u64,
}

impl SlidingWindow {
    /// Create a new sliding window.
    pub fn new(window_ms: u64, bucket_count: usize) -> Self {
        let bucket_count = bucket_count.max(1);
        let bucket_duration_ms = window_ms / bucket_count as u64;
        Self {
            window_ms,
            bucket_count,
            buckets: vec![0; bucket_count],
            bucket_times: vec![0; bucket_count],
            bucket_duration_ms: bucket_duration_ms.max(1),
            current_index: 0,
            total_events: 0,
        }
    }

    /// Record an event at the given timestamp.
    pub fn record(&mut self, now_ms: u64) {
        self.advance(now_ms);
        self.buckets[self.current_index] += 1;
        self.total_events += 1;
    }

    /// Get the total count within the current window.
    pub fn count(&self, now_ms: u64) -> u64 {
        let cutoff = now_ms.saturating_sub(self.window_ms);
        let mut total = 0u64;
        for i in 0..self.bucket_count {
            if self.bucket_times[i] >= cutoff {
                total += self.buckets[i];
            }
        }
        total
    }

    /// Get the rate (events per second) within the current window.
    pub fn rate_per_second(&self, now_ms: u64) -> f64 {
        let count = self.count(now_ms);
        if self.window_ms == 0 {
            return 0.0;
        }
        count as f64 / (self.window_ms as f64 / 1000.0)
    }

    /// Total events tracked (lifetime).
    pub fn total_events(&self) -> u64 {
        self.total_events
    }

    /// Reset all counters.
    pub fn reset(&mut self) {
        self.buckets.fill(0);
        self.bucket_times.fill(0);
        self.current_index = 0;
    }

    /// Advance the window to the current time, clearing stale buckets.
    fn advance(&mut self, now_ms: u64) {
        let current_bucket_time = self.bucket_times[self.current_index];

        if current_bucket_time == 0 {
            self.bucket_times[self.current_index] = now_ms;
            return;
        }

        let elapsed = now_ms.saturating_sub(current_bucket_time);
        if elapsed >= self.bucket_duration_ms {
            let buckets_to_advance =
                ((elapsed / self.bucket_duration_ms) as usize).min(self.bucket_count);

            for _ in 0..buckets_to_advance {
                self.current_index = (self.current_index + 1) % self.bucket_count;
                self.buckets[self.current_index] = 0;
                self.bucket_times[self.current_index] = now_ms;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_window() {
        let window = SlidingWindow::new(10_000, 10);
        assert_eq!(window.count(0), 0);
        assert_eq!(window.total_events(), 0);
    }

    #[test]
    fn test_record_and_count() {
        let mut window = SlidingWindow::new(10_000, 10);
        window.record(1000);
        window.record(2000);
        window.record(3000);
        assert_eq!(window.count(5000), 3);
        assert_eq!(window.total_events(), 3);
    }

    #[test]
    fn test_events_expire() {
        let mut window = SlidingWindow::new(5000, 5);
        window.record(1000);
        window.record(2000);
        window.record(3000);

        // At time 8000, events at 1000 and 2000 should be expired (cutoff = 3000)
        let count = window.count(8000);
        assert!(count <= 3, "some events should have expired");
    }

    #[test]
    fn test_rate_per_second() {
        let mut window = SlidingWindow::new(10_000, 10);
        for t in 0..10 {
            window.record(t * 1000);
        }
        let rate = window.rate_per_second(10_000);
        assert!((rate - 1.0).abs() < 0.01, "rate should be ~1.0 per second");
    }

    #[test]
    fn test_reset() {
        let mut window = SlidingWindow::new(10_000, 10);
        window.record(1000);
        window.record(2000);
        window.reset();
        assert_eq!(window.count(3000), 0);
    }

    #[test]
    fn test_single_bucket() {
        let mut window = SlidingWindow::new(5000, 1);
        window.record(1000);
        window.record(2000);
        assert_eq!(window.count(3000), 2);
    }

    #[test]
    fn test_rapid_events() {
        let mut window = SlidingWindow::new(1000, 10);
        for i in 0..100 {
            window.record(i * 5); // every 5ms
        }
        assert_eq!(window.total_events(), 100);
    }
}
