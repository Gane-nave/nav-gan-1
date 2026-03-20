//! Batch collector — accumulates events and flushes them as a batch.

/// A batch collector that accumulates events and flushes when a threshold is met.
pub struct BatchCollector {
    /// Maximum batch size.
    max_size: usize,
    /// Maximum time to wait before flushing (ms).
    max_wait_ms: u64,
    /// Current batch of events (stored as byte payloads).
    batch: Vec<Vec<u8>>,
    /// Timestamp of the first event in the current batch.
    first_event_ms: Option<u64>,
    /// Total events collected (lifetime).
    total_events: u64,
    /// Total flushes (lifetime).
    total_flushes: u64,
    /// Total bytes flushed (lifetime).
    total_bytes_flushed: u64,
}

impl BatchCollector {
    /// Create a new batch collector.
    pub fn new(max_size: usize, max_wait_ms: u64) -> Self {
        Self {
            max_size,
            max_wait_ms,
            batch: Vec::with_capacity(max_size),
            first_event_ms: None,
            total_events: 0,
            total_flushes: 0,
            total_bytes_flushed: 0,
        }
    }

    /// Add an event to the batch. Returns true if the batch is full and should be flushed.
    pub fn add(&mut self, payload: Vec<u8>, now_ms: u64) -> bool {
        if self.first_event_ms.is_none() {
            self.first_event_ms = Some(now_ms);
        }
        self.batch.push(payload);
        self.total_events += 1;
        self.batch.len() >= self.max_size
    }

    /// Check if the batch should be flushed based on time.
    pub fn should_flush_by_time(&self, now_ms: u64) -> bool {
        match self.first_event_ms {
            Some(first) => {
                now_ms.saturating_sub(first) >= self.max_wait_ms && !self.batch.is_empty()
            }
            None => false,
        }
    }

    /// Check if the batch should be flushed (by size or time).
    pub fn should_flush(&self, now_ms: u64) -> bool {
        self.batch.len() >= self.max_size || self.should_flush_by_time(now_ms)
    }

    /// Flush the batch, returning the collected events.
    pub fn flush(&mut self) -> Vec<Vec<u8>> {
        let batch = std::mem::take(&mut self.batch);
        let bytes: u64 = batch.iter().map(|e| e.len() as u64).sum();
        self.total_bytes_flushed += bytes;
        self.total_flushes += 1;
        self.first_event_ms = None;
        self.batch = Vec::with_capacity(self.max_size);
        batch
    }

    /// Current batch size.
    pub fn current_size(&self) -> usize {
        self.batch.len()
    }

    /// Check if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.batch.is_empty()
    }

    /// Maximum batch size.
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Total events collected (lifetime).
    pub fn total_events(&self) -> u64 {
        self.total_events
    }

    /// Total flushes (lifetime).
    pub fn total_flushes(&self) -> u64 {
        self.total_flushes
    }

    /// Total bytes flushed (lifetime).
    pub fn total_bytes_flushed(&self) -> u64 {
        self.total_bytes_flushed
    }

    /// Average batch size (events per flush).
    pub fn avg_batch_size(&self) -> f64 {
        if self.total_flushes == 0 {
            return 0.0;
        }
        self.total_events as f64 / self.total_flushes as f64
    }

    /// Fill ratio of current batch (0.0 to 1.0).
    pub fn fill_ratio(&self) -> f64 {
        if self.max_size == 0 {
            return 1.0;
        }
        self.batch.len() as f64 / self.max_size as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_add_and_flush() {
        let mut bc = BatchCollector::new(3, 10_000);
        assert!(!bc.add(vec![1], 100));
        assert!(!bc.add(vec![2], 200));
        assert!(bc.add(vec![3], 300)); // full
        assert_eq!(bc.current_size(), 3);

        let batch = bc.flush();
        assert_eq!(batch.len(), 3);
        assert!(bc.is_empty());
        assert_eq!(bc.total_flushes(), 1);
    }

    #[test]
    fn test_batch_time_flush() {
        let mut bc = BatchCollector::new(100, 5000);
        bc.add(vec![1], 1000);
        assert!(!bc.should_flush_by_time(3000)); // 2s < 5s
        assert!(bc.should_flush_by_time(6000)); // 5s = max_wait
    }

    #[test]
    fn test_batch_empty_no_time_flush() {
        let bc = BatchCollector::new(100, 5000);
        assert!(!bc.should_flush_by_time(10000)); // empty batch
    }

    #[test]
    fn test_batch_bytes_tracking() {
        let mut bc = BatchCollector::new(2, 10_000);
        bc.add(vec![1, 2, 3], 100); // 3 bytes
        bc.add(vec![4, 5], 200); // 2 bytes
        bc.flush();
        assert_eq!(bc.total_bytes_flushed(), 5);
    }

    #[test]
    fn test_batch_fill_ratio() {
        let mut bc = BatchCollector::new(4, 10_000);
        assert!((bc.fill_ratio() - 0.0).abs() < f64::EPSILON);
        bc.add(vec![1], 100);
        bc.add(vec![2], 200);
        assert!((bc.fill_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_batch_avg_size() {
        let mut bc = BatchCollector::new(2, 10_000);
        bc.add(vec![1], 100);
        bc.add(vec![2], 200);
        bc.flush(); // 2 events
        bc.add(vec![3], 300);
        bc.add(vec![4], 400);
        bc.flush(); // 2 events
        assert!((bc.avg_batch_size() - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_batch_stats() {
        let mut bc = BatchCollector::new(3, 10_000);
        bc.add(vec![1], 100);
        bc.add(vec![2], 200);
        bc.add(vec![3], 300);
        bc.flush();
        assert_eq!(bc.total_events(), 3);
        assert_eq!(bc.total_flushes(), 1);
    }

    #[test]
    fn test_batch_zero_max() {
        let mut bc = BatchCollector::new(0, 10_000);
        assert!(bc.add(vec![1], 100)); // immediately full
        assert!((bc.fill_ratio() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_batch_should_flush_combined() {
        let mut bc = BatchCollector::new(10, 5000);
        bc.add(vec![1], 1000);
        assert!(!bc.should_flush(2000)); // neither size nor time
        assert!(bc.should_flush(6000)); // time trigger
    }
}
