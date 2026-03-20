//! Time-series storage — ordered data points with downsampling and range queries.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single data point in a time series.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub tags: HashMap<String, String>,
}

/// Aggregation function for downsampling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateFunc {
    Mean,
    Min,
    Max,
    Sum,
    Count,
    Last,
}

/// A downsampled bucket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub value: f64,
    pub count: usize,
}

/// Time-series store with named series, range queries, and downsampling.
pub struct TimeSeriesStore {
    series: RwLock<HashMap<String, Vec<DataPoint>>>,
    max_points_per_series: usize,
}

impl TimeSeriesStore {
    /// Create a new store with a per-series point limit.
    pub fn new(max_points_per_series: usize) -> Self {
        Self {
            series: RwLock::new(HashMap::new()),
            max_points_per_series,
        }
    }

    /// Append a data point to a series (maintains chronological order).
    pub fn append(&self, series_name: &str, point: DataPoint) {
        let mut store = self.series.write();
        let series = store.entry(series_name.to_string()).or_default();
        series.push(point);
        // Evict oldest if over limit
        if series.len() > self.max_points_per_series {
            let drain_count = (self.max_points_per_series / 10).max(1);
            series.drain(..drain_count);
        }
    }

    /// Query points in a time range (inclusive).
    pub fn range(
        &self,
        series_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Vec<DataPoint> {
        let store = self.series.read();
        store
            .get(series_name)
            .map(|pts| {
                pts.iter()
                    .filter(|p| (from..=to).contains(&p.timestamp))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get the latest N points from a series.
    pub fn latest(&self, series_name: &str, n: usize) -> Vec<DataPoint> {
        let store = self.series.read();
        store
            .get(series_name)
            .map(|pts| {
                let start = pts.len().saturating_sub(n);
                pts[start..].to_vec()
            })
            .unwrap_or_default()
    }

    /// Downsample a range into fixed-width buckets.
    pub fn downsample(
        &self,
        series_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        bucket_width_secs: i64,
        func: AggregateFunc,
    ) -> Vec<Bucket> {
        let points = self.range(series_name, from, to);
        if points.is_empty() || bucket_width_secs <= 0 {
            return Vec::new();
        }

        let mut buckets: Vec<Bucket> = Vec::new();
        let mut bucket_start = from;

        while bucket_start < to {
            let bucket_end = bucket_start + chrono::Duration::seconds(bucket_width_secs);
            let bucket_points: Vec<f64> = points
                .iter()
                .filter(|p| p.timestamp >= bucket_start && p.timestamp < bucket_end)
                .map(|p| p.value)
                .collect();

            if !bucket_points.is_empty() {
                let value = match func {
                    AggregateFunc::Mean => {
                        bucket_points.iter().sum::<f64>() / bucket_points.len() as f64
                    }
                    AggregateFunc::Min => {
                        bucket_points.iter().cloned().fold(f64::INFINITY, f64::min)
                    }
                    AggregateFunc::Max => bucket_points
                        .iter()
                        .cloned()
                        .fold(f64::NEG_INFINITY, f64::max),
                    AggregateFunc::Sum => bucket_points.iter().sum(),
                    AggregateFunc::Count => bucket_points.len() as f64,
                    AggregateFunc::Last => *bucket_points.last().unwrap(),
                };
                buckets.push(Bucket {
                    start: bucket_start,
                    end: bucket_end,
                    value,
                    count: bucket_points.len(),
                });
            }

            bucket_start = bucket_end;
        }

        buckets
    }

    /// Count total points in a series.
    pub fn len(&self, series_name: &str) -> usize {
        self.series
            .read()
            .get(series_name)
            .map(|s| s.len())
            .unwrap_or(0)
    }

    /// Check if a series exists.
    pub fn contains(&self, series_name: &str) -> bool {
        self.series.read().contains_key(series_name)
    }

    /// List all series names.
    pub fn series_names(&self) -> Vec<String> {
        self.series.read().keys().cloned().collect()
    }

    /// Drop an entire series.
    pub fn drop_series(&self, series_name: &str) -> bool {
        self.series.write().remove(series_name).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn make_point(ts: DateTime<Utc>, value: f64) -> DataPoint {
        DataPoint {
            timestamp: ts,
            value,
            tags: HashMap::new(),
        }
    }

    #[test]
    fn test_append_and_query() {
        let store = TimeSeriesStore::new(1000);
        let now = Utc::now();
        store.append("speed", make_point(now, 60.0));
        store.append("speed", make_point(now + Duration::seconds(1), 65.0));
        store.append("speed", make_point(now + Duration::seconds(2), 70.0));
        assert_eq!(store.len("speed"), 3);
    }

    #[test]
    fn test_range_query() {
        let store = TimeSeriesStore::new(1000);
        let base = Utc::now();
        for i in 0..10 {
            store.append("temp", make_point(base + Duration::seconds(i), i as f64));
        }
        let from = base + Duration::seconds(3);
        let to = base + Duration::seconds(7);
        let result = store.range("temp", from, to);
        assert_eq!(result.len(), 5); // 3,4,5,6,7
    }

    #[test]
    fn test_latest() {
        let store = TimeSeriesStore::new(1000);
        let base = Utc::now();
        for i in 0..10 {
            store.append(
                "alt",
                make_point(base + Duration::seconds(i), i as f64 * 100.0),
            );
        }
        let last3 = store.latest("alt", 3);
        assert_eq!(last3.len(), 3);
        assert!((last3[0].value - 700.0).abs() < f64::EPSILON);
        assert!((last3[2].value - 900.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_downsample_mean() {
        let store = TimeSeriesStore::new(1000);
        let base = Utc::now();
        // 6 points, 2 per bucket (3-second buckets)
        for i in 0..6 {
            store.append(
                "fuel",
                make_point(base + Duration::seconds(i), (i + 1) as f64),
            );
        }
        let buckets = store.downsample(
            "fuel",
            base,
            base + Duration::seconds(6),
            3,
            AggregateFunc::Mean,
        );
        assert_eq!(buckets.len(), 2);
        assert!((buckets[0].value - 2.0).abs() < f64::EPSILON); // mean(1,2,3)
        assert!((buckets[1].value - 5.0).abs() < f64::EPSILON); // mean(4,5,6)
    }

    #[test]
    fn test_downsample_max() {
        let store = TimeSeriesStore::new(1000);
        let base = Utc::now();
        for i in 0..4 {
            store.append(
                "rpm",
                make_point(base + Duration::seconds(i), (i * 1000) as f64),
            );
        }
        let buckets = store.downsample(
            "rpm",
            base,
            base + Duration::seconds(4),
            2,
            AggregateFunc::Max,
        );
        assert_eq!(buckets.len(), 2);
        assert!((buckets[0].value - 1000.0).abs() < f64::EPSILON);
        assert!((buckets[1].value - 3000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_downsample_count() {
        let store = TimeSeriesStore::new(1000);
        let base = Utc::now();
        for i in 0..5 {
            store.append("events", make_point(base + Duration::seconds(i), 1.0));
        }
        let buckets = store.downsample(
            "events",
            base,
            base + Duration::seconds(6),
            3,
            AggregateFunc::Count,
        );
        assert_eq!(buckets.len(), 2);
        assert!((buckets[0].value - 3.0).abs() < f64::EPSILON);
        assert!((buckets[1].value - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_eviction() {
        let store = TimeSeriesStore::new(5);
        let base = Utc::now();
        for i in 0..10 {
            store.append("s", make_point(base + Duration::seconds(i), i as f64));
        }
        assert!(store.len("s") <= 5);
    }

    #[test]
    fn test_series_management() {
        let store = TimeSeriesStore::new(1000);
        store.append("a", make_point(Utc::now(), 1.0));
        store.append("b", make_point(Utc::now(), 2.0));
        assert!(store.contains("a"));
        assert!(!store.contains("c"));
        let mut names = store.series_names();
        names.sort();
        assert_eq!(names, vec!["a", "b"]);
        assert!(store.drop_series("a"));
        assert!(!store.contains("a"));
    }

    #[test]
    fn test_empty_queries() {
        let store = TimeSeriesStore::new(1000);
        assert!(store.range("nope", Utc::now(), Utc::now()).is_empty());
        assert!(store.latest("nope", 5).is_empty());
        assert_eq!(store.len("nope"), 0);
    }
}
