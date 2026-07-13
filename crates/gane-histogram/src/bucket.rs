//! Histogram buckets — fixed-width and exponential bucket strategies.

/// A single histogram bucket.
#[derive(Debug, Clone)]
pub struct Bucket {
    /// Lower bound (inclusive).
    pub lower: f64,
    /// Upper bound (exclusive).
    pub upper: f64,
    /// Count of values in this bucket.
    pub count: u64,
}

impl Bucket {
    /// Create a new bucket.
    pub fn new(lower: f64, upper: f64) -> Self {
        Self {
            lower,
            upper,
            count: 0,
        }
    }

    /// Check if a value falls in this bucket.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.lower && value < self.upper
    }

    /// Width of the bucket.
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }

    /// Midpoint of the bucket.
    pub fn midpoint(&self) -> f64 {
        (self.lower + self.upper) / 2.0
    }
}

/// A histogram with fixed-width buckets.
#[derive(Debug, Clone)]
pub struct Histogram {
    buckets: Vec<Bucket>,
    /// Count of values below the lowest bucket.
    underflow: u64,
    /// Count of values at or above the highest bucket.
    overflow: u64,
    /// Total number of recorded values.
    total_count: u64,
    /// Sum of all recorded values.
    total_sum: f64,
    /// Minimum recorded value.
    min: f64,
    /// Maximum recorded value.
    max: f64,
}

impl Histogram {
    /// Create a histogram with fixed-width buckets.
    pub fn with_fixed_buckets(min: f64, max: f64, num_buckets: usize) -> Self {
        let width = (max - min) / num_buckets as f64;
        let buckets = (0..num_buckets)
            .map(|i| {
                let lower = min + i as f64 * width;
                let upper = min + (i + 1) as f64 * width;
                Bucket::new(lower, upper)
            })
            .collect();

        Self {
            buckets,
            underflow: 0,
            overflow: 0,
            total_count: 0,
            total_sum: 0.0,
            min: f64::MAX,
            max: f64::MIN,
        }
    }

    /// Create a histogram with exponential buckets (base^0, base^1, ...).
    pub fn with_exponential_buckets(base: f64, num_buckets: usize) -> Self {
        let buckets = (0..num_buckets)
            .map(|i| {
                let lower = base.powi(i as i32);
                let upper = base.powi(i as i32 + 1);
                Bucket::new(lower, upper)
            })
            .collect();

        Self {
            buckets,
            underflow: 0,
            overflow: 0,
            total_count: 0,
            total_sum: 0.0,
            min: f64::MAX,
            max: f64::MIN,
        }
    }

    /// Record a value.
    pub fn record(&mut self, value: f64) {
        self.total_count += 1;
        self.total_sum += value;
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }

        let mut placed = false;
        for bucket in &mut self.buckets {
            if bucket.contains(value) {
                bucket.count += 1;
                placed = true;
                break;
            }
        }

        if !placed {
            if let Some(first) = self.buckets.first() {
                if value < first.lower {
                    self.underflow += 1;
                } else {
                    self.overflow += 1;
                }
            } else {
                self.overflow += 1;
            }
        }
    }

    /// Get the buckets.
    pub fn buckets(&self) -> &[Bucket] {
        &self.buckets
    }

    /// Total recorded values.
    pub fn count(&self) -> u64 {
        self.total_count
    }

    /// Sum of all recorded values.
    pub fn sum(&self) -> f64 {
        self.total_sum
    }

    /// Mean of recorded values.
    pub fn mean(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }
        self.total_sum / self.total_count as f64
    }

    /// Minimum recorded value (or f64::MAX if none).
    pub fn min_value(&self) -> f64 {
        self.min
    }

    /// Maximum recorded value (or f64::MIN if none).
    pub fn max_value(&self) -> f64 {
        self.max
    }

    /// Underflow count.
    pub fn underflow(&self) -> u64 {
        self.underflow
    }

    /// Overflow count.
    pub fn overflow(&self) -> u64 {
        self.overflow
    }

    /// Find the bucket with the most entries.
    pub fn mode_bucket(&self) -> Option<&Bucket> {
        self.buckets.iter().max_by_key(|b| b.count)
    }

    /// Reset all counts.
    pub fn reset(&mut self) {
        for bucket in &mut self.buckets {
            bucket.count = 0;
        }
        self.underflow = 0;
        self.overflow = 0;
        self.total_count = 0;
        self.total_sum = 0.0;
        self.min = f64::MAX;
        self.max = f64::MIN;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_bucket_creation() {
        let hist = Histogram::with_fixed_buckets(0.0, 100.0, 10);
        assert_eq!(hist.buckets().len(), 10);
        assert!((hist.buckets()[0].lower - 0.0).abs() < 1e-10);
        assert!((hist.buckets()[0].upper - 10.0).abs() < 1e-10);
        assert!((hist.buckets()[9].upper - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_record_values() {
        let mut hist = Histogram::with_fixed_buckets(0.0, 100.0, 10);
        hist.record(5.0);
        hist.record(15.0);
        hist.record(5.5);

        assert_eq!(hist.count(), 3);
        assert_eq!(hist.buckets()[0].count, 2); // 0-10
        assert_eq!(hist.buckets()[1].count, 1); // 10-20
    }

    #[test]
    fn test_underflow_overflow() {
        let mut hist = Histogram::with_fixed_buckets(10.0, 20.0, 2);
        hist.record(5.0); // underflow
        hist.record(25.0); // overflow
        hist.record(15.0); // in range

        assert_eq!(hist.underflow(), 1);
        assert_eq!(hist.overflow(), 1);
        assert_eq!(hist.count(), 3);
    }

    #[test]
    fn test_mean() {
        let mut hist = Histogram::with_fixed_buckets(0.0, 100.0, 10);
        hist.record(10.0);
        hist.record(20.0);
        hist.record(30.0);
        assert!((hist.mean() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_min_max() {
        let mut hist = Histogram::with_fixed_buckets(0.0, 100.0, 10);
        hist.record(42.0);
        hist.record(7.0);
        hist.record(99.0);
        assert!((hist.min_value() - 7.0).abs() < 1e-10);
        assert!((hist.max_value() - 99.0).abs() < 1e-10);
    }

    #[test]
    fn test_exponential_buckets() {
        let hist = Histogram::with_exponential_buckets(2.0, 4);
        assert_eq!(hist.buckets().len(), 4);
        assert!((hist.buckets()[0].lower - 1.0).abs() < 1e-10); // 2^0
        assert!((hist.buckets()[0].upper - 2.0).abs() < 1e-10); // 2^1
        assert!((hist.buckets()[3].upper - 16.0).abs() < 1e-10); // 2^4
    }

    #[test]
    fn test_mode_bucket() {
        let mut hist = Histogram::with_fixed_buckets(0.0, 30.0, 3);
        hist.record(5.0); // bucket 0
        hist.record(15.0); // bucket 1
        hist.record(16.0); // bucket 1
        hist.record(25.0); // bucket 2

        let mode = hist.mode_bucket().unwrap();
        assert!((mode.lower - 10.0).abs() < 1e-10);
        assert_eq!(mode.count, 2);
    }

    #[test]
    fn test_reset() {
        let mut hist = Histogram::with_fixed_buckets(0.0, 10.0, 2);
        hist.record(3.0);
        hist.record(7.0);
        hist.reset();
        assert_eq!(hist.count(), 0);
        assert_eq!(hist.buckets()[0].count, 0);
    }

    #[test]
    fn test_bucket_properties() {
        let b = Bucket::new(10.0, 20.0);
        assert!(b.contains(15.0));
        assert!(!b.contains(5.0));
        assert!(!b.contains(20.0)); // upper is exclusive
        assert!((b.width() - 10.0).abs() < 1e-10);
        assert!((b.midpoint() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_histogram_mean() {
        let hist = Histogram::with_fixed_buckets(0.0, 10.0, 2);
        assert!((hist.mean() - 0.0).abs() < 1e-10);
    }
}
