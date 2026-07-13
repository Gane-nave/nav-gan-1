//! Interval representation for range queries.

/// A closed interval [low, high] with an associated value.
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    /// Lower bound (inclusive).
    low: i64,
    /// Upper bound (inclusive).
    high: i64,
    /// Associated label/value.
    label: String,
}

impl Interval {
    /// Create a new interval. Panics if low > high.
    pub fn new(low: i64, high: i64, label: &str) -> Self {
        assert!(low <= high, "low ({}) must be <= high ({})", low, high);
        Self {
            low,
            high,
            label: label.to_string(),
        }
    }

    /// Get the lower bound.
    pub fn low(&self) -> i64 {
        self.low
    }

    /// Get the upper bound.
    pub fn high(&self) -> i64 {
        self.high
    }

    /// Get the label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the length of the interval.
    pub fn length(&self) -> i64 {
        self.high.saturating_sub(self.low)
    }

    /// Check if a point is contained in this interval.
    pub fn contains_point(&self, point: i64) -> bool {
        (self.low..=self.high).contains(&point)
    }

    /// Check if this interval overlaps with another.
    pub fn overlaps(&self, other: &Interval) -> bool {
        self.low <= other.high && other.low <= self.high
    }

    /// Check if this interval fully contains another.
    pub fn contains_interval(&self, other: &Interval) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    /// Get the midpoint of the interval.
    pub fn midpoint(&self) -> i64 {
        self.low + (self.high - self.low) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_creation() {
        let iv = Interval::new(10, 20, "test");
        assert_eq!(iv.low(), 10);
        assert_eq!(iv.high(), 20);
        assert_eq!(iv.label(), "test");
    }

    #[test]
    fn test_point_interval() {
        let iv = Interval::new(5, 5, "point");
        assert_eq!(iv.length(), 0);
        assert!(iv.contains_point(5));
        assert!(!iv.contains_point(4));
    }

    #[test]
    #[should_panic]
    fn test_invalid_interval() {
        Interval::new(20, 10, "bad");
    }

    #[test]
    fn test_contains_point() {
        let iv = Interval::new(10, 20, "range");
        assert!(iv.contains_point(10));
        assert!(iv.contains_point(15));
        assert!(iv.contains_point(20));
        assert!(!iv.contains_point(9));
        assert!(!iv.contains_point(21));
    }

    #[test]
    fn test_overlaps() {
        let a = Interval::new(10, 20, "a");
        let b = Interval::new(15, 25, "b");
        let c = Interval::new(21, 30, "c");
        let d = Interval::new(20, 21, "d");
        assert!(a.overlaps(&b));
        assert!(!a.overlaps(&c));
        assert!(a.overlaps(&d)); // touching at boundary
    }

    #[test]
    fn test_contains_interval() {
        let outer = Interval::new(0, 100, "outer");
        let inner = Interval::new(10, 20, "inner");
        let partial = Interval::new(50, 150, "partial");
        assert!(outer.contains_interval(&inner));
        assert!(!outer.contains_interval(&partial));
        assert!(!inner.contains_interval(&outer));
    }

    #[test]
    fn test_length() {
        assert_eq!(Interval::new(0, 10, "x").length(), 10);
        assert_eq!(Interval::new(-5, 5, "y").length(), 10);
    }

    #[test]
    fn test_midpoint() {
        assert_eq!(Interval::new(0, 10, "x").midpoint(), 5);
        assert_eq!(Interval::new(10, 20, "y").midpoint(), 15);
    }

    #[test]
    fn test_clone_and_eq() {
        let a = Interval::new(1, 2, "a");
        let b = a.clone();
        assert_eq!(a, b);
    }
}
