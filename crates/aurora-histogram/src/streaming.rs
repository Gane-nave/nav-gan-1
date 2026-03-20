//! Streaming statistics — online algorithms for mean, variance, min, max.

/// Welford's online algorithm for streaming mean and variance.
#[derive(Debug, Clone)]
pub struct StreamingStats {
    count: u64,
    mean: f64,
    m2: f64,
    min: f64,
    max: f64,
    sum: f64,
}

impl StreamingStats {
    /// Create a new streaming stats accumulator.
    pub fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            sum: 0.0,
        }
    }

    /// Add a value.
    pub fn push(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }
        let delta = value - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = value - self.mean;
        self.m2 += delta * delta2;
    }

    /// Number of values.
    pub fn count(&self) -> u64 {
        self.count
    }

    /// Running mean.
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// Sum of all values.
    pub fn sum(&self) -> f64 {
        self.sum
    }

    /// Population variance.
    pub fn variance(&self) -> f64 {
        if self.count < 2 {
            return 0.0;
        }
        self.m2 / self.count as f64
    }

    /// Sample variance (Bessel's correction).
    pub fn sample_variance(&self) -> f64 {
        if self.count < 2 {
            return 0.0;
        }
        self.m2 / (self.count - 1) as f64
    }

    /// Population standard deviation.
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    /// Sample standard deviation.
    pub fn sample_std_dev(&self) -> f64 {
        self.sample_variance().sqrt()
    }

    /// Minimum value.
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Maximum value.
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Range (max - min).
    pub fn range(&self) -> f64 {
        if self.count == 0 {
            return 0.0;
        }
        self.max - self.min
    }

    /// Reset all statistics.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Merge another StreamingStats into this one.
    pub fn merge(&mut self, other: &StreamingStats) {
        if other.count == 0 {
            return;
        }
        if self.count == 0 {
            *self = other.clone();
            return;
        }

        let combined_count = self.count + other.count;
        let delta = other.mean - self.mean;
        let combined_mean = (self.sum + other.sum) / combined_count as f64;
        let combined_m2 = self.m2
            + other.m2
            + delta * delta * (self.count as f64 * other.count as f64) / combined_count as f64;

        self.count = combined_count;
        self.mean = combined_mean;
        self.m2 = combined_m2;
        self.sum += other.sum;
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}

impl Default for StreamingStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Exponentially weighted moving average (EWMA).
#[derive(Debug, Clone)]
pub struct Ewma {
    alpha: f64,
    value: Option<f64>,
}

impl Ewma {
    /// Create an EWMA with the given smoothing factor (0 < alpha <= 1).
    /// Higher alpha = more weight on recent values.
    pub fn new(alpha: f64) -> Self {
        Self { alpha, value: None }
    }

    /// Add a new data point.
    pub fn push(&mut self, value: f64) {
        self.value = Some(match self.value {
            None => value,
            Some(prev) => self.alpha * value + (1.0 - self.alpha) * prev,
        });
    }

    /// Get the current EWMA value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the EWMA.
    pub fn reset(&mut self) {
        self.value = None;
    }

    /// Smoothing factor.
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_mean() {
        let mut stats = StreamingStats::new();
        stats.push(10.0);
        stats.push(20.0);
        stats.push(30.0);
        assert!((stats.mean() - 20.0).abs() < 1e-10);
        assert_eq!(stats.count(), 3);
    }

    #[test]
    fn test_streaming_variance() {
        let mut stats = StreamingStats::new();
        for v in [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0] {
            stats.push(v);
        }
        assert!((stats.variance() - 4.0).abs() < 0.01);
        assert!((stats.std_dev() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_streaming_min_max() {
        let mut stats = StreamingStats::new();
        stats.push(5.0);
        stats.push(1.0);
        stats.push(9.0);
        assert!((stats.min() - 1.0).abs() < 1e-10);
        assert!((stats.max() - 9.0).abs() < 1e-10);
        assert!((stats.range() - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_streaming_empty() {
        let stats = StreamingStats::new();
        assert_eq!(stats.count(), 0);
        assert!((stats.mean() - 0.0).abs() < 1e-10);
        assert!((stats.variance() - 0.0).abs() < 1e-10);
        assert!((stats.range() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_streaming_merge() {
        let mut a = StreamingStats::new();
        a.push(1.0);
        a.push(2.0);

        let mut b = StreamingStats::new();
        b.push(3.0);
        b.push(4.0);

        a.merge(&b);
        assert_eq!(a.count(), 4);
        assert!((a.mean() - 2.5).abs() < 1e-10);
        assert!((a.min() - 1.0).abs() < 1e-10);
        assert!((a.max() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_streaming_reset() {
        let mut stats = StreamingStats::new();
        stats.push(42.0);
        stats.reset();
        assert_eq!(stats.count(), 0);
    }

    #[test]
    fn test_ewma_basic() {
        let mut ewma = Ewma::new(0.5);
        assert!(ewma.value().is_none());
        ewma.push(10.0);
        assert!((ewma.value().unwrap() - 10.0).abs() < 1e-10);
        ewma.push(20.0);
        // 0.5 * 20 + 0.5 * 10 = 15
        assert!((ewma.value().unwrap() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_ewma_high_alpha() {
        let mut ewma = Ewma::new(1.0);
        ewma.push(10.0);
        ewma.push(20.0);
        // alpha=1 means fully weight recent value
        assert!((ewma.value().unwrap() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_ewma_reset() {
        let mut ewma = Ewma::new(0.5);
        ewma.push(10.0);
        ewma.reset();
        assert!(ewma.value().is_none());
    }

    #[test]
    fn test_sample_variance() {
        let mut stats = StreamingStats::new();
        stats.push(2.0);
        stats.push(4.0);
        // sample variance = (2-3)^2 + (4-3)^2 / (2-1) = 2
        assert!((stats.sample_variance() - 2.0).abs() < 0.01);
    }
}
