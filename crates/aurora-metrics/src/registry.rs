//! Metric registry — thread-safe storage for all metric types.

use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;

/// A monotonically increasing counter.
#[derive(Debug)]
pub struct Counter {
    name: String,
    help: String,
    value: AtomicU64,
}

impl Counter {
    pub fn new(name: &str, help: &str) -> Self {
        Self {
            name: name.to_string(),
            help: help.to_string(),
            value: AtomicU64::new(0),
        }
    }

    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_by(&self, n: u64) {
        self.value.fetch_add(n, Ordering::Relaxed);
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn help(&self) -> &str {
        &self.help
    }
}

/// A gauge that can go up and down.
#[derive(Debug)]
pub struct Gauge {
    name: String,
    help: String,
    value: AtomicI64,
}

impl Gauge {
    pub fn new(name: &str, help: &str) -> Self {
        Self {
            name: name.to_string(),
            help: help.to_string(),
            value: AtomicI64::new(0),
        }
    }

    pub fn set(&self, v: i64) {
        self.value.store(v, Ordering::Relaxed);
    }

    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec(&self) {
        self.value.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn get(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn help(&self) -> &str {
        &self.help
    }
}

/// A histogram that tracks value distributions using configurable buckets.
#[derive(Debug)]
pub struct Histogram {
    name: String,
    help: String,
    buckets: Vec<f64>,
    counts: Vec<AtomicU64>,
    sum: AtomicU64, // stored as f64 bits
    count: AtomicU64,
}

impl Histogram {
    pub fn new(name: &str, help: &str, buckets: Vec<f64>) -> Self {
        let counts = buckets.iter().map(|_| AtomicU64::new(0)).collect();
        Self {
            name: name.to_string(),
            help: help.to_string(),
            buckets,
            counts,
            sum: AtomicU64::new(f64::to_bits(0.0)),
            count: AtomicU64::new(0),
        }
    }

    /// Default histogram buckets for latency in seconds.
    pub fn default_buckets() -> Vec<f64> {
        vec![
            0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ]
    }

    pub fn observe(&self, value: f64) {
        self.count.fetch_add(1, Ordering::Relaxed);

        // Atomically add to sum using CAS loop
        loop {
            let current_bits = self.sum.load(Ordering::Relaxed);
            let current = f64::from_bits(current_bits);
            let new = current + value;
            let new_bits = f64::to_bits(new);
            if self
                .sum
                .compare_exchange_weak(current_bits, new_bits, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }

        // Find the first bucket that contains this value and increment only that one.
        // The export layer converts these per-bucket counts into cumulative counts.
        for (i, bucket) in self.buckets.iter().enumerate() {
            if value <= *bucket {
                self.counts[i].fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn help(&self) -> &str {
        &self.help
    }

    pub fn buckets(&self) -> &[f64] {
        &self.buckets
    }

    pub fn bucket_counts(&self) -> Vec<u64> {
        self.counts
            .iter()
            .map(|c| c.load(Ordering::Relaxed))
            .collect()
    }

    pub fn sum(&self) -> f64 {
        f64::from_bits(self.sum.load(Ordering::Relaxed))
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
}

/// Central metric registry holding all counters, gauges, and histograms.
#[derive(Debug, Default)]
pub struct MetricRegistry {
    counters: RwLock<BTreeMap<String, Arc<Counter>>>,
    gauges: RwLock<BTreeMap<String, Arc<Gauge>>>,
    histograms: RwLock<BTreeMap<String, Arc<Histogram>>>,
}

impl MetricRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register or retrieve a counter.
    pub fn counter(&self, name: &str, help: &str) -> Arc<Counter> {
        let read = self.counters.read();
        if let Some(c) = read.get(name) {
            return c.clone();
        }
        drop(read);

        let mut write = self.counters.write();
        write
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Counter::new(name, help)))
            .clone()
    }

    /// Register or retrieve a gauge.
    pub fn gauge(&self, name: &str, help: &str) -> Arc<Gauge> {
        let read = self.gauges.read();
        if let Some(g) = read.get(name) {
            return g.clone();
        }
        drop(read);

        let mut write = self.gauges.write();
        write
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Gauge::new(name, help)))
            .clone()
    }

    /// Register or retrieve a histogram.
    pub fn histogram(&self, name: &str, help: &str, buckets: Vec<f64>) -> Arc<Histogram> {
        let read = self.histograms.read();
        if let Some(h) = read.get(name) {
            return h.clone();
        }
        drop(read);

        let mut write = self.histograms.write();
        write
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Histogram::new(name, help, buckets)))
            .clone()
    }

    /// Returns all counters in sorted order.
    pub fn all_counters(&self) -> Vec<Arc<Counter>> {
        self.counters.read().values().cloned().collect()
    }

    /// Returns all gauges in sorted order.
    pub fn all_gauges(&self) -> Vec<Arc<Gauge>> {
        self.gauges.read().values().cloned().collect()
    }

    /// Returns all histograms in sorted order.
    pub fn all_histograms(&self) -> Vec<Arc<Histogram>> {
        self.histograms.read().values().cloned().collect()
    }

    /// Total number of registered metrics.
    pub fn metric_count(&self) -> usize {
        self.counters.read().len() + self.gauges.read().len() + self.histograms.read().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_increments() {
        let c = Counter::new("test_total", "A test counter");
        assert_eq!(c.get(), 0);
        c.inc();
        assert_eq!(c.get(), 1);
        c.inc_by(5);
        assert_eq!(c.get(), 6);
    }

    #[test]
    fn gauge_set_inc_dec() {
        let g = Gauge::new("test_gauge", "A test gauge");
        assert_eq!(g.get(), 0);
        g.set(42);
        assert_eq!(g.get(), 42);
        g.inc();
        assert_eq!(g.get(), 43);
        g.dec();
        assert_eq!(g.get(), 42);
    }

    #[test]
    fn histogram_observe_distributes_to_buckets() {
        let h = Histogram::new("test_hist", "A test histogram", vec![0.1, 0.5, 1.0]);
        h.observe(0.05);
        h.observe(0.3);
        h.observe(0.8);
        h.observe(2.0);

        assert_eq!(h.count(), 4);
        let counts = h.bucket_counts();
        // Per-bucket (non-cumulative): 0.05 in [0.1], 0.3 in [0.5], 0.8 in [1.0], 2.0 in none
        assert_eq!(counts[0], 1); // 0.05 <= 0.1
        assert_eq!(counts[1], 1); // 0.3 <= 0.5
        assert_eq!(counts[2], 1); // 0.8 <= 1.0
                                  // sum = 0.05 + 0.3 + 0.8 + 2.0 = 3.15
        assert!((h.sum() - 3.15).abs() < 0.001);
    }

    #[test]
    fn registry_creates_and_retrieves_same_metric() {
        let r = MetricRegistry::new();
        let c1 = r.counter("http_requests_total", "Total HTTP requests");
        let c2 = r.counter("http_requests_total", "Total HTTP requests");
        c1.inc();
        assert_eq!(c2.get(), 1); // Same underlying counter
    }

    #[test]
    fn registry_metric_count() {
        let r = MetricRegistry::new();
        assert_eq!(r.metric_count(), 0);
        r.counter("a", "");
        r.gauge("b", "");
        r.histogram("c", "", vec![1.0]);
        assert_eq!(r.metric_count(), 3);
    }

    #[test]
    fn histogram_default_buckets() {
        let buckets = Histogram::default_buckets();
        assert_eq!(buckets.len(), 12);
        assert!((buckets[0] - 0.001).abs() < f64::EPSILON);
        assert!((buckets[11] - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn gauge_negative_values() {
        let g = Gauge::new("temp", "Temperature");
        g.set(-10);
        assert_eq!(g.get(), -10);
        g.dec();
        assert_eq!(g.get(), -11);
    }

    #[test]
    fn counter_name_and_help() {
        let c = Counter::new("test_total", "A test counter");
        assert_eq!(c.name(), "test_total");
        assert_eq!(c.help(), "A test counter");
    }

    #[test]
    fn all_counters_sorted() {
        let r = MetricRegistry::new();
        r.counter("z_counter", "");
        r.counter("a_counter", "");
        let all = r.all_counters();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].name(), "a_counter");
        assert_eq!(all[1].name(), "z_counter");
    }
}
