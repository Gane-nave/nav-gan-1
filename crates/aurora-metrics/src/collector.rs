//! Pre-defined metric collectors for AURORA NAV subsystems.

use crate::registry::MetricRegistry;
use std::sync::Arc;

/// Standard navigation metrics.
pub struct NavigationMetrics {
    pub gnss_satellites_tracked: Arc<crate::registry::Gauge>,
    pub gnss_fix_acquisitions_total: Arc<crate::registry::Counter>,
    pub fusion_updates_total: Arc<crate::registry::Counter>,
    pub fusion_position_uncertainty_m: Arc<crate::registry::Gauge>,
    pub integrity_level_changes_total: Arc<crate::registry::Counter>,
    pub continuity_mode_switches_total: Arc<crate::registry::Counter>,
    pub events_published_total: Arc<crate::registry::Counter>,
    pub telemetry_samples_buffered: Arc<crate::registry::Gauge>,
    pub api_requests_total: Arc<crate::registry::Counter>,
    pub api_request_duration_seconds: Arc<crate::registry::Histogram>,
    pub api_errors_total: Arc<crate::registry::Counter>,
    pub route_calculations_total: Arc<crate::registry::Counter>,
    pub route_calculation_duration_seconds: Arc<crate::registry::Histogram>,
    pub map_tiles_cached: Arc<crate::registry::Gauge>,
    pub offline_queue_depth: Arc<crate::registry::Gauge>,
    pub edge_inference_total: Arc<crate::registry::Counter>,
    pub edge_inference_duration_seconds: Arc<crate::registry::Histogram>,
}

impl NavigationMetrics {
    /// Register all standard navigation metrics with the given registry.
    pub fn register(registry: &MetricRegistry) -> Self {
        Self {
            gnss_satellites_tracked: registry.gauge(
                "aurora_gnss_satellites_tracked",
                "Number of currently tracked GNSS satellites",
            ),
            gnss_fix_acquisitions_total: registry.counter(
                "aurora_gnss_fix_acquisitions_total",
                "Total number of GNSS fix acquisitions",
            ),
            fusion_updates_total: registry.counter(
                "aurora_fusion_updates_total",
                "Total EKF fusion update cycles",
            ),
            fusion_position_uncertainty_m: registry.gauge(
                "aurora_fusion_position_uncertainty_metres",
                "Current horizontal position uncertainty in metres",
            ),
            integrity_level_changes_total: registry.counter(
                "aurora_integrity_level_changes_total",
                "Total integrity level transitions",
            ),
            continuity_mode_switches_total: registry.counter(
                "aurora_continuity_mode_switches_total",
                "Total continuity mode transitions",
            ),
            events_published_total: registry.counter(
                "aurora_events_published_total",
                "Total events published to the event bus",
            ),
            telemetry_samples_buffered: registry.gauge(
                "aurora_telemetry_samples_buffered",
                "Number of telemetry samples in the ring buffer",
            ),
            api_requests_total: registry.counter(
                "aurora_api_requests_total",
                "Total HTTP API requests served",
            ),
            api_request_duration_seconds: registry.histogram(
                "aurora_api_request_duration_seconds",
                "API request latency in seconds",
                crate::registry::Histogram::default_buckets(),
            ),
            api_errors_total: registry.counter(
                "aurora_api_errors_total",
                "Total HTTP API error responses (4xx/5xx)",
            ),
            route_calculations_total: registry.counter(
                "aurora_route_calculations_total",
                "Total route calculations performed",
            ),
            route_calculation_duration_seconds: registry.histogram(
                "aurora_route_calculation_duration_seconds",
                "Route calculation latency in seconds",
                vec![0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0],
            ),
            map_tiles_cached: registry.gauge(
                "aurora_map_tiles_cached",
                "Number of map tiles in the tile cache",
            ),
            offline_queue_depth: registry.gauge(
                "aurora_offline_queue_depth",
                "Number of operations queued for offline sync",
            ),
            edge_inference_total: registry.counter(
                "aurora_edge_inference_total",
                "Total edge inference operations",
            ),
            edge_inference_duration_seconds: registry.histogram(
                "aurora_edge_inference_duration_seconds",
                "Edge inference latency in seconds",
                vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0],
            ),
        }
    }

    /// Returns the total number of registered metrics.
    pub fn metric_count(&self) -> usize {
        17 // Fixed set of standard metrics
    }
}

/// System-level metrics (process info).
pub struct SystemMetrics {
    pub uptime_seconds: Arc<crate::registry::Gauge>,
    pub active_connections: Arc<crate::registry::Gauge>,
    pub build_info: Arc<crate::registry::Gauge>,
}

impl SystemMetrics {
    /// Register system-level metrics.
    pub fn register(registry: &MetricRegistry) -> Self {
        let build_info = registry.gauge("aurora_build_info", "Build information (always 1)");
        build_info.set(1);

        Self {
            uptime_seconds: registry.gauge(
                "aurora_uptime_seconds",
                "Time since server start in seconds",
            ),
            active_connections: registry.gauge(
                "aurora_active_connections",
                "Number of active HTTP connections",
            ),
            build_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_metrics_register_all() {
        let registry = MetricRegistry::new();
        let nav = NavigationMetrics::register(&registry);
        assert_eq!(nav.metric_count(), 17);
        // Verify metrics work
        nav.gnss_satellites_tracked.set(12);
        assert_eq!(nav.gnss_satellites_tracked.get(), 12);
        nav.api_requests_total.inc();
        nav.api_requests_total.inc();
        assert_eq!(nav.api_requests_total.get(), 2);
    }

    #[test]
    fn system_metrics_register() {
        let registry = MetricRegistry::new();
        let sys = SystemMetrics::register(&registry);
        assert_eq!(sys.build_info.get(), 1);
        sys.uptime_seconds.set(3600);
        assert_eq!(sys.uptime_seconds.get(), 3600);
    }

    #[test]
    fn histogram_metrics_record_latency() {
        let registry = MetricRegistry::new();
        let nav = NavigationMetrics::register(&registry);
        nav.api_request_duration_seconds.observe(0.002);
        nav.api_request_duration_seconds.observe(0.15);
        nav.api_request_duration_seconds.observe(3.5);
        assert_eq!(nav.api_request_duration_seconds.count(), 3);
        assert!((nav.api_request_duration_seconds.sum() - 3.652).abs() < 0.001);
    }

    #[test]
    fn metrics_all_in_registry() {
        let registry = MetricRegistry::new();
        let _nav = NavigationMetrics::register(&registry);
        let _sys = SystemMetrics::register(&registry);
        // 17 nav + 3 sys = 20 total metrics
        assert_eq!(registry.metric_count(), 20);
    }

    #[test]
    fn edge_inference_histogram_custom_buckets() {
        let registry = MetricRegistry::new();
        let nav = NavigationMetrics::register(&registry);
        assert_eq!(nav.edge_inference_duration_seconds.buckets().len(), 7);
        nav.edge_inference_duration_seconds.observe(0.003);
        let counts = nav.edge_inference_duration_seconds.bucket_counts();
        // 0.003 <= 0.005 bucket (index 1)
        assert_eq!(counts[1], 1);
        // 0.003 > 0.001 bucket (index 0)
        assert_eq!(counts[0], 0);
    }
}
