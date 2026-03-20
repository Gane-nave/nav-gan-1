//! Unified service registry wiring infrastructure crates together.

use aurora_backpressure::controller::{FlowControlConfig, FlowController};
use aurora_bloom::filter::BloomFilter;
use aurora_bounded_queue::{BoundedQueue, PushResult};
use aurora_btree::BTree;
use aurora_cache::ttl::TtlCache;
use aurora_circuit::breaker::{BreakerConfig, CircuitBreaker, State as BreakerState};
use aurora_clock::HybridClock;
use aurora_errors::recovery::DegradationManager;
use aurora_features::flags::FlagRegistry;
use aurora_histogram::bucket::Histogram;
use aurora_lru::cache::LruCache;
use aurora_perf::profiler::Profiler;
use aurora_ratelimit::bucket::{BucketConfig, BucketLimiter};
use aurora_retry::backoff::BackoffStrategy;
use aurora_retry::policy::{RetryPolicy, RetryPolicyConfig};
use aurora_snapshot::capture::SnapshotStore;
use aurora_structured_log::filter::FilterChain;

/// Unified service registry providing access to all infrastructure services.
///
/// This registry is the single point of access for the navigation pipeline
/// to use caching, circuit breaking, rate limiting, feature flags, and
/// other infrastructure capabilities.
pub struct ServiceRegistry {
    /// TTL cache for frequently-accessed routes.
    pub route_cache: TtlCache<Vec<u8>>,
    /// LRU cache for recent position lookups.
    pub position_cache: LruCache<String>,
    /// Circuit breaker for external service calls (map tiles, corrections).
    pub external_circuit: CircuitBreaker,
    /// Retry policy for transient failures.
    pub retry_policy: RetryPolicy,
    /// Bloom filter for seen-waypoint deduplication.
    pub waypoint_filter: BloomFilter,
    /// Bucket-based rate limiter for API requests.
    pub api_limiter: BucketLimiter,
    /// Feature flag registry for runtime toggles.
    pub features: FlagRegistry,
    /// Snapshot store for pipeline state checkpoints.
    pub snapshots: SnapshotStore,
    /// Flow controller for backpressure.
    pub flow_controller: FlowController,
    /// Bounded queue for pending navigation requests.
    pub nav_queue: BoundedQueue<NavigationRequest>,
    /// Histogram for latency tracking.
    pub latency_histogram: Histogram,
    /// Log filter chain.
    pub log_filter: FilterChain,
    /// Degradation manager for error recovery.
    pub degradation: DegradationManager,
    /// Performance profiler.
    pub profiler: Profiler,
    /// B-Tree index for spatial lookups.
    pub spatial_index: BTree<u64, (f64, f64)>,
    /// Hybrid logical clock for distributed event ordering.
    pub clock: HybridClock,
}

/// A navigation request queued for processing.
#[derive(Debug, Clone)]
pub struct NavigationRequest {
    /// Request identifier.
    pub id: u64,
    /// Origin latitude.
    pub origin_lat: f64,
    /// Origin longitude.
    pub origin_lon: f64,
    /// Destination latitude.
    pub dest_lat: f64,
    /// Destination longitude.
    pub dest_lon: f64,
    /// Timestamp (ms since epoch).
    pub timestamp_ms: u64,
}

impl ServiceRegistry {
    /// Create a new service registry with default settings.
    pub fn new() -> Self {
        Self {
            route_cache: TtlCache::new(10_000, 300_000),
            position_cache: LruCache::new(1_000, None),
            external_circuit: CircuitBreaker::new(BreakerConfig {
                failure_threshold: 5,
                success_threshold: 3,
                open_duration_ms: 30_000,
                half_open_max_requests: 3,
            }),
            retry_policy: RetryPolicy::new(RetryPolicyConfig {
                max_attempts: 3,
                backoff: BackoffStrategy::Exponential {
                    initial_ms: 100,
                    multiplier: 2.0,
                    max_ms: 5_000,
                },
                retry_on_timeout: true,
                retry_on_transient: true,
            }),
            waypoint_filter: BloomFilter::new(100_000, 7),
            api_limiter: BucketLimiter::new(BucketConfig::new(1_000, 100.0)),
            features: FlagRegistry::new(),
            snapshots: SnapshotStore::new(50),
            flow_controller: FlowController::new(FlowControlConfig::default()),
            nav_queue: BoundedQueue::new(1_000),
            latency_histogram: Histogram::with_fixed_buckets(0.0, 10_000.0, 100),
            log_filter: FilterChain::new(),
            degradation: DegradationManager::new(),
            profiler: Profiler::new(10_000),
            spatial_index: BTree::new(16),
            clock: HybridClock::new(0),
        }
    }

    /// Create with custom capacity settings.
    pub fn with_capacity(
        route_cache_size: usize,
        position_cache_size: usize,
        queue_capacity: usize,
    ) -> Self {
        Self {
            route_cache: TtlCache::new(route_cache_size, 300_000),
            position_cache: LruCache::new(position_cache_size, None),
            external_circuit: CircuitBreaker::new(BreakerConfig {
                failure_threshold: 5,
                success_threshold: 3,
                open_duration_ms: 30_000,
                half_open_max_requests: 3,
            }),
            retry_policy: RetryPolicy::new(RetryPolicyConfig {
                max_attempts: 3,
                backoff: BackoffStrategy::Exponential {
                    initial_ms: 100,
                    multiplier: 2.0,
                    max_ms: 5_000,
                },
                retry_on_timeout: true,
                retry_on_transient: true,
            }),
            waypoint_filter: BloomFilter::new(100_000, 7),
            api_limiter: BucketLimiter::new(BucketConfig::new(1_000, 100.0)),
            features: FlagRegistry::new(),
            snapshots: SnapshotStore::new(50),
            flow_controller: FlowController::new(FlowControlConfig::default()),
            nav_queue: BoundedQueue::new(queue_capacity),
            latency_histogram: Histogram::with_fixed_buckets(0.0, 10_000.0, 100),
            log_filter: FilterChain::new(),
            degradation: DegradationManager::new(),
            profiler: Profiler::new(10_000),
            spatial_index: BTree::new(16),
            clock: HybridClock::new(0),
        }
    }

    /// Check if the circuit breaker allows external calls.
    ///
    /// This is a read-only check that does NOT trigger state transitions.
    /// Use [`try_external_call`] for the full Open→HalfOpen recovery path.
    pub fn can_call_external(&self) -> bool {
        matches!(
            self.external_circuit.state(),
            BreakerState::Closed | BreakerState::HalfOpen
        )
    }

    /// Attempt an external call through the circuit breaker.
    ///
    /// Unlike [`can_call_external`], this method triggers state transitions
    /// (e.g. Open→HalfOpen after the cooldown elapses) and should be called
    /// before every external request.
    pub fn try_external_call(&mut self) -> bool {
        self.external_circuit.allow_request(Self::wall_clock_ms())
    }

    /// Record a successful external call.
    pub fn record_external_success(&mut self) {
        self.external_circuit.record_success();
    }

    /// Record a failed external call.
    pub fn record_external_failure(&mut self) {
        self.external_circuit.record_failure(Self::wall_clock_ms());
    }

    /// Check if a feature flag is enabled.
    pub fn is_feature_enabled(&mut self, flag: &str) -> bool {
        self.features.is_enabled(flag)
    }

    /// Enqueue a navigation request. Returns false if queue is full.
    pub fn enqueue_nav_request(&mut self, req: NavigationRequest) -> bool {
        matches!(self.nav_queue.push(req), PushResult::Ok)
    }

    /// Dequeue the next navigation request.
    pub fn dequeue_nav_request(&mut self) -> Option<NavigationRequest> {
        self.nav_queue.pop()
    }

    /// Record a latency observation.
    pub fn record_latency(&mut self, micros: f64) {
        self.latency_histogram.record(micros);
    }

    /// Get the current timestamp from the hybrid logical clock.
    pub fn now(&mut self) -> aurora_clock::HlcTimestamp {
        self.clock.now(Self::wall_clock_ms())
    }

    /// Check if waypoint was already visited (probabilistic).
    pub fn was_waypoint_visited(&self, waypoint_id: &[u8]) -> bool {
        self.waypoint_filter.contains_no_track(waypoint_id)
    }

    /// Mark a waypoint as visited.
    pub fn mark_waypoint_visited(&mut self, waypoint_id: &[u8]) {
        self.waypoint_filter.insert(waypoint_id);
    }

    /// Take a snapshot of the pipeline state.
    pub fn take_snapshot(&mut self, timestamp_ms: u64) -> u64 {
        let snap = self.snapshots.create(timestamp_ms);
        snap.id
    }

    /// Get the number of pending navigation requests.
    pub fn pending_requests(&self) -> usize {
        self.nav_queue.len()
    }

    /// Get summary statistics.
    pub fn stats(&self) -> RegistryStats {
        RegistryStats {
            route_cache_size: self.route_cache.len(),
            position_cache_size: self.position_cache.len(),
            circuit_breaker_closed: matches!(self.external_circuit.state(), BreakerState::Closed),
            pending_nav_requests: self.nav_queue.len(),
            latency_count: self.latency_histogram.count(),
            snapshots_taken: self.snapshots.count(),
            waypoint_filter_count: self.waypoint_filter.inserted(),
        }
    }

    fn wall_clock_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary statistics for the service registry.
#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub route_cache_size: usize,
    pub position_cache_size: usize,
    pub circuit_breaker_closed: bool,
    pub pending_nav_requests: usize,
    pub latency_count: u64,
    pub snapshots_taken: usize,
    pub waypoint_filter_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_default_registry() {
        let reg = ServiceRegistry::new();
        assert!(reg.can_call_external());
        assert_eq!(reg.pending_requests(), 0);
    }

    #[test]
    fn test_default_impl() {
        let reg = ServiceRegistry::default();
        assert!(reg.can_call_external());
    }

    #[test]
    fn test_with_capacity() {
        let reg = ServiceRegistry::with_capacity(100, 50, 10);
        assert!(reg.can_call_external());
        assert_eq!(reg.pending_requests(), 0);
    }

    #[test]
    fn test_enqueue_dequeue_nav_request() {
        let mut reg = ServiceRegistry::new();
        let req = NavigationRequest {
            id: 1,
            origin_lat: 32.0,
            origin_lon: 34.0,
            dest_lat: 31.0,
            dest_lon: 35.0,
            timestamp_ms: 1000,
        };
        assert!(reg.enqueue_nav_request(req));
        assert_eq!(reg.pending_requests(), 1);
        let popped = reg.dequeue_nav_request().unwrap();
        assert_eq!(popped.id, 1);
        assert_eq!(reg.pending_requests(), 0);
    }

    #[test]
    fn test_circuit_breaker_integration() {
        let mut reg = ServiceRegistry::new();
        assert!(reg.can_call_external());
        for _ in 0..10 {
            reg.record_external_failure();
        }
        assert!(!reg.can_call_external());
    }

    #[test]
    fn test_circuit_breaker_recovery_via_try() {
        let mut reg = ServiceRegistry::new();
        // Trip the breaker open
        for _ in 0..10 {
            reg.record_external_failure();
        }
        assert!(!reg.can_call_external());
        // try_external_call uses allow_request which triggers Open→HalfOpen
        // after cooldown. Since last_failure_ms is wall-clock recent, the
        // breaker stays Open and rejects. We verify the method is callable
        // and returns false (cooldown not elapsed yet).
        assert!(!reg.try_external_call());
    }

    #[test]
    fn test_waypoint_filter() {
        let mut reg = ServiceRegistry::new();
        assert!(!reg.was_waypoint_visited(b"wp_001"));
        reg.mark_waypoint_visited(b"wp_001");
        assert!(reg.was_waypoint_visited(b"wp_001"));
    }

    #[test]
    fn test_latency_recording() {
        let mut reg = ServiceRegistry::new();
        reg.record_latency(100.0);
        reg.record_latency(200.0);
        reg.record_latency(300.0);
        assert_eq!(reg.latency_histogram.count(), 3);
    }

    #[test]
    fn test_snapshot() {
        let mut reg = ServiceRegistry::new();
        let id = reg.take_snapshot(1000);
        assert!(id > 0);
        assert_eq!(reg.snapshots.count(), 1);
    }

    #[test]
    fn test_stats() {
        let reg = ServiceRegistry::new();
        let stats = reg.stats();
        assert_eq!(stats.route_cache_size, 0);
        assert_eq!(stats.position_cache_size, 0);
        assert!(stats.circuit_breaker_closed);
        assert_eq!(stats.pending_nav_requests, 0);
    }

    #[test]
    fn test_clock_produces_timestamps() {
        let mut reg = ServiceRegistry::new();
        let t1 = reg.now();
        let t2 = reg.now();
        assert!(t1 <= t2);
    }

    #[test]
    fn test_empty_dequeue() {
        let mut reg = ServiceRegistry::new();
        assert!(reg.dequeue_nav_request().is_none());
    }
}
