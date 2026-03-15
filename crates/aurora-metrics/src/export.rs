//! Prometheus text exposition format exporter.

use crate::registry::MetricRegistry;

/// Render all metrics in Prometheus text exposition format.
pub fn render_prometheus(registry: &MetricRegistry) -> String {
    let mut out = String::with_capacity(4096);

    // Counters
    for counter in registry.all_counters() {
        out.push_str(&format!("# HELP {} {}\n", counter.name(), counter.help()));
        out.push_str(&format!("# TYPE {} counter\n", counter.name()));
        out.push_str(&format!("{} {}\n", counter.name(), counter.get()));
    }

    // Gauges
    for gauge in registry.all_gauges() {
        out.push_str(&format!("# HELP {} {}\n", gauge.name(), gauge.help()));
        out.push_str(&format!("# TYPE {} gauge\n", gauge.name()));
        out.push_str(&format!("{} {}\n", gauge.name(), gauge.get()));
    }

    // Histograms
    for hist in registry.all_histograms() {
        out.push_str(&format!("# HELP {} {}\n", hist.name(), hist.help()));
        out.push_str(&format!("# TYPE {} histogram\n", hist.name()));

        let counts = hist.bucket_counts();
        let mut cumulative = 0u64;
        for (i, bucket) in hist.buckets().iter().enumerate() {
            cumulative += counts[i];
            out.push_str(&format!(
                "{}_bucket{{le=\"{}\"}} {}\n",
                hist.name(),
                format_bucket_bound(*bucket),
                cumulative
            ));
        }
        // +Inf bucket
        out.push_str(&format!(
            "{}_bucket{{le=\"+Inf\"}} {}\n",
            hist.name(),
            hist.count()
        ));
        out.push_str(&format!("{}_sum {}\n", hist.name(), hist.sum()));
        out.push_str(&format!("{}_count {}\n", hist.name(), hist.count()));
    }

    out
}

/// Format a bucket bound for Prometheus output.
fn format_bucket_bound(v: f64) -> String {
    if v == f64::INFINITY {
        "+Inf".to_string()
    } else if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{:.1}", v)
    } else {
        format!("{}", v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::MetricRegistry;

    #[test]
    fn render_counter() {
        let registry = MetricRegistry::new();
        let c = registry.counter("http_total", "Total HTTP requests");
        c.inc();
        c.inc();
        let output = render_prometheus(&registry);
        assert!(output.contains("# HELP http_total Total HTTP requests"));
        assert!(output.contains("# TYPE http_total counter"));
        assert!(output.contains("http_total 2"));
    }

    #[test]
    fn render_gauge() {
        let registry = MetricRegistry::new();
        let g = registry.gauge("temperature", "Current temp");
        g.set(42);
        let output = render_prometheus(&registry);
        assert!(output.contains("# TYPE temperature gauge"));
        assert!(output.contains("temperature 42"));
    }

    #[test]
    fn render_histogram_with_buckets() {
        let registry = MetricRegistry::new();
        let h = registry.histogram("latency", "Request latency", vec![0.1, 0.5, 1.0]);
        h.observe(0.05); // → bucket 0.1
        h.observe(0.3); // → bucket 0.5
        h.observe(2.0); // → no bucket (above all)
        let output = render_prometheus(&registry);
        assert!(output.contains("# TYPE latency histogram"));
        // Cumulative: bucket[0.1]=1, bucket[0.5]=1+1=2, bucket[1.0]=2+0=2
        assert!(output.contains("latency_bucket{le=\"0.1\"} 1"));
        assert!(output.contains("latency_bucket{le=\"0.5\"} 2"));
        assert!(output.contains("latency_bucket{le=\"1.0\"} 2"));
        assert!(output.contains("latency_bucket{le=\"+Inf\"} 3"));
        assert!(output.contains("latency_count 3"));
    }

    #[test]
    fn render_empty_registry() {
        let registry = MetricRegistry::new();
        let output = render_prometheus(&registry);
        assert!(output.is_empty());
    }

    #[test]
    fn render_mixed_metrics() {
        let registry = MetricRegistry::new();
        registry.counter("a_count", "counter A").inc();
        registry.gauge("b_gauge", "gauge B").set(10);
        registry
            .histogram("c_hist", "hist C", vec![1.0])
            .observe(0.5);
        let output = render_prometheus(&registry);
        assert!(output.contains("a_count 1"));
        assert!(output.contains("b_gauge 10"));
        assert!(output.contains("c_hist_count 1"));
    }

    #[test]
    fn format_bucket_bound_integer() {
        assert_eq!(format_bucket_bound(1.0), "1.0");
        assert_eq!(format_bucket_bound(10.0), "10.0");
    }

    #[test]
    fn format_bucket_bound_fractional() {
        assert_eq!(format_bucket_bound(0.001), "0.001");
        assert_eq!(format_bucket_bound(0.25), "0.25");
    }

    #[test]
    fn format_bucket_bound_infinity() {
        assert_eq!(format_bucket_bound(f64::INFINITY), "+Inf");
    }

    #[test]
    fn cumulative_histogram_buckets() {
        let registry = MetricRegistry::new();
        let h = registry.histogram("req", "Requests", vec![0.1, 0.5, 1.0, 5.0]);
        // All 3 values fall into all buckets from their threshold up
        h.observe(0.05); // <= 0.1, 0.5, 1.0, 5.0
        h.observe(0.05); // <= 0.1, 0.5, 1.0, 5.0
        h.observe(0.3); // <= 0.5, 1.0, 5.0
        let output = render_prometheus(&registry);
        // Cumulative: bucket 0.1 = 2, bucket 0.5 = 2+1=3, bucket 1.0 = 3, bucket 5.0 = 3
        assert!(output.contains("req_bucket{le=\"0.1\"} 2"));
        assert!(output.contains("req_bucket{le=\"0.5\"} 3"));
        assert!(output.contains("req_bucket{le=\"1.0\"} 3"));
        assert!(output.contains("req_bucket{le=\"5.0\"} 3"));
    }
}
