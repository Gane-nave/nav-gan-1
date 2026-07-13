//! External smart-city data integration — adapters for ingesting data from
//! municipal APIs, IoT platforms, and third-party feeds, with normalization
//! and freshness tracking.

use chrono::{DateTime, Duration, Utc};
use gane_core::types::EntityId;
use std::collections::HashMap;
use tracing::{debug, info, warn};

// ---------------------------------------------------------------------------
// Data source types
// ---------------------------------------------------------------------------

/// Kind of external data source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataSourceKind {
    /// Municipal open-data API.
    MunicipalApi,
    /// IoT sensor platform (e.g., environmental sensors).
    IoTSensorPlatform,
    /// Traffic management centre feed.
    TrafficManagementCentre,
    /// Public transport real-time feed (GTFS-RT).
    PublicTransportFeed,
    /// Weather service.
    WeatherService,
    /// Air quality monitoring network.
    AirQualityNetwork,
    /// Parking operator API.
    ParkingOperator,
    /// EV charging network.
    ChargingNetwork,
    /// Emergency services feed.
    EmergencyServicesFeed,
}

/// Health status of a data source connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceHealth {
    /// Receiving data within expected cadence.
    Healthy,
    /// Data is arriving but with higher latency than expected.
    Degraded,
    /// No data received within the staleness window.
    Stale,
    /// Source is unreachable or returning errors.
    Offline,
}

/// Registration of an external data source.
#[derive(Debug, Clone)]
pub struct DataSource {
    pub id: EntityId,
    pub name: String,
    pub kind: DataSourceKind,
    /// Expected update interval (seconds).
    pub expected_interval_s: f64,
    /// Maximum age before data is considered stale (seconds).
    pub staleness_threshold_s: f64,
    /// Whether the source is enabled.
    pub enabled: bool,
    pub registered_at: DateTime<Utc>,
}

/// A normalized data record received from an external source.
#[derive(Debug, Clone)]
pub struct NormalizedRecord {
    pub id: EntityId,
    pub source_id: EntityId,
    pub record_type: String,
    pub payload: serde_json::Value,
    pub raw_timestamp: DateTime<Utc>,
    pub ingested_at: DateTime<Utc>,
    pub quality_score: f64,
}

/// Statistics for a data source.
#[derive(Debug, Clone)]
pub struct SourceStatistics {
    pub source_id: EntityId,
    pub total_records: u64,
    pub records_last_hour: u64,
    pub avg_latency_ms: f64,
    pub error_count: u64,
    pub last_received_at: Option<DateTime<Utc>>,
    pub health: SourceHealth,
}

// ---------------------------------------------------------------------------
// Integration hub
// ---------------------------------------------------------------------------

/// Internal tracking state for a source.
struct SourceTracker {
    source: DataSource,
    records: Vec<NormalizedRecord>,
    total_records: u64,
    error_count: u64,
    last_received_at: Option<DateTime<Utc>>,
    latency_samples: Vec<f64>,
}

/// The integration hub manages connections to external smart-city data
/// sources, normalizes incoming data, tracks freshness and health, and
/// provides a unified query interface.
pub struct IntegrationHub {
    sources: HashMap<EntityId, SourceTracker>,
    /// Maximum records to retain per source (ring buffer).
    max_records_per_source: usize,
}

impl IntegrationHub {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            max_records_per_source: 1000,
        }
    }

    pub fn with_max_records(max_records_per_source: usize) -> Self {
        Self {
            sources: HashMap::new(),
            max_records_per_source,
        }
    }

    // -----------------------------------------------------------------------
    // Source management
    // -----------------------------------------------------------------------

    /// Register an external data source.
    pub fn register_source(&mut self, source: DataSource) {
        info!(id = %source.id, name = %source.name, kind = ?source.kind, "data source registered");
        let id = source.id;
        self.sources.insert(
            id,
            SourceTracker {
                source,
                records: Vec::new(),
                total_records: 0,
                error_count: 0,
                last_received_at: None,
                latency_samples: Vec::new(),
            },
        );
    }

    /// Enable a data source.
    pub fn enable_source(&mut self, source_id: &EntityId) -> bool {
        if let Some(tracker) = self.sources.get_mut(source_id) {
            tracker.source.enabled = true;
            debug!(id = %source_id, "source enabled");
            return true;
        }
        false
    }

    /// Disable a data source.
    pub fn disable_source(&mut self, source_id: &EntityId) -> bool {
        if let Some(tracker) = self.sources.get_mut(source_id) {
            tracker.source.enabled = false;
            debug!(id = %source_id, "source disabled");
            return true;
        }
        false
    }

    /// Get a registered source.
    pub fn get_source(&self, source_id: &EntityId) -> Option<&DataSource> {
        self.sources.get(source_id).map(|t| &t.source)
    }

    /// Number of registered sources.
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    /// Get all sources by kind.
    pub fn sources_by_kind(&self, kind: DataSourceKind) -> Vec<&DataSource> {
        self.sources
            .values()
            .filter(|t| t.source.kind == kind)
            .map(|t| &t.source)
            .collect()
    }

    // -----------------------------------------------------------------------
    // Data ingestion
    // -----------------------------------------------------------------------

    /// Ingest a normalized record from an external source.
    pub fn ingest(&mut self, record: NormalizedRecord) -> bool {
        let source_id = record.source_id;
        let Some(tracker) = self.sources.get_mut(&source_id) else {
            warn!(source = %source_id, "ingested record for unknown source");
            return false;
        };

        if !tracker.source.enabled {
            debug!(source = %source_id, "ignoring record from disabled source");
            return false;
        }

        // Track latency.
        let latency_ms = (record.ingested_at - record.raw_timestamp).num_milliseconds() as f64;
        tracker.latency_samples.push(latency_ms);
        if tracker.latency_samples.len() > 100 {
            tracker.latency_samples.remove(0);
        }

        tracker.last_received_at = Some(record.ingested_at);
        tracker.total_records += 1;

        // Ring buffer: evict oldest if full.
        if tracker.records.len() >= self.max_records_per_source {
            tracker.records.remove(0);
        }
        tracker.records.push(record);

        true
    }

    /// Record an error from a data source.
    pub fn record_error(&mut self, source_id: &EntityId) {
        if let Some(tracker) = self.sources.get_mut(source_id) {
            tracker.error_count += 1;
            warn!(source = %source_id, errors = tracker.error_count, "source error recorded");
        }
    }

    // -----------------------------------------------------------------------
    // Health & statistics
    // -----------------------------------------------------------------------

    /// Assess the health of a data source.
    pub fn assess_health(&self, source_id: &EntityId) -> SourceHealth {
        let Some(tracker) = self.sources.get(source_id) else {
            return SourceHealth::Offline;
        };

        if !tracker.source.enabled {
            return SourceHealth::Offline;
        }

        let Some(last) = tracker.last_received_at else {
            return SourceHealth::Stale;
        };

        let age_s = (Utc::now() - last).num_seconds() as f64;

        if age_s > tracker.source.staleness_threshold_s {
            SourceHealth::Stale
        } else if age_s > tracker.source.expected_interval_s * 2.0 {
            SourceHealth::Degraded
        } else {
            SourceHealth::Healthy
        }
    }

    /// Get statistics for a data source.
    pub fn statistics(&self, source_id: &EntityId) -> Option<SourceStatistics> {
        let tracker = self.sources.get(source_id)?;
        let now = Utc::now();

        let records_last_hour = tracker
            .records
            .iter()
            .filter(|r| r.ingested_at > now - Duration::hours(1))
            .count() as u64;

        let avg_latency = if tracker.latency_samples.is_empty() {
            0.0
        } else {
            tracker.latency_samples.iter().sum::<f64>() / tracker.latency_samples.len() as f64
        };

        Some(SourceStatistics {
            source_id: *source_id,
            total_records: tracker.total_records,
            records_last_hour,
            avg_latency_ms: avg_latency,
            error_count: tracker.error_count,
            last_received_at: tracker.last_received_at,
            health: self.assess_health(source_id),
        })
    }

    /// Get the most recent records from a source.
    pub fn recent_records(&self, source_id: &EntityId, limit: usize) -> Vec<&NormalizedRecord> {
        let Some(tracker) = self.sources.get(source_id) else {
            return Vec::new();
        };
        tracker.records.iter().rev().take(limit).collect()
    }

    /// Get all unhealthy sources.
    pub fn unhealthy_sources(&self) -> Vec<(&DataSource, SourceHealth)> {
        self.sources
            .iter()
            .filter_map(|(id, tracker)| {
                let health = self.assess_health(id);
                if health != SourceHealth::Healthy {
                    Some((&tracker.source, health))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Count of healthy sources.
    pub fn healthy_count(&self) -> usize {
        self.sources
            .keys()
            .filter(|id| self.assess_health(id) == SourceHealth::Healthy)
            .count()
    }
}

impl Default for IntegrationHub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_source(kind: DataSourceKind) -> DataSource {
        DataSource {
            id: EntityId::new(),
            name: "TestSource".into(),
            kind,
            expected_interval_s: 60.0,
            staleness_threshold_s: 300.0,
            enabled: true,
            registered_at: Utc::now(),
        }
    }

    fn make_record(source_id: EntityId) -> NormalizedRecord {
        let now = Utc::now();
        NormalizedRecord {
            id: EntityId::new(),
            source_id,
            record_type: "test".into(),
            payload: serde_json::json!({"value": 42}),
            raw_timestamp: now - Duration::milliseconds(50),
            ingested_at: now,
            quality_score: 0.95,
        }
    }

    #[test]
    fn register_and_query_source() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::MunicipalApi);
        let id = src.id;
        hub.register_source(src);

        assert_eq!(hub.source_count(), 1);
        assert!(hub.get_source(&id).is_some());
    }

    #[test]
    fn enable_disable_source() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::IoTSensorPlatform);
        let id = src.id;
        hub.register_source(src);

        assert!(hub.disable_source(&id));
        assert!(!hub.get_source(&id).unwrap().enabled);
        assert!(hub.enable_source(&id));
        assert!(hub.get_source(&id).unwrap().enabled);
    }

    #[test]
    fn ingest_record_success() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::WeatherService);
        let src_id = src.id;
        hub.register_source(src);

        let record = make_record(src_id);
        assert!(hub.ingest(record));

        let stats = hub.statistics(&src_id).unwrap();
        assert_eq!(stats.total_records, 1);
    }

    #[test]
    fn ingest_rejected_for_disabled_source() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::WeatherService);
        let src_id = src.id;
        hub.register_source(src);
        hub.disable_source(&src_id);

        assert!(!hub.ingest(make_record(src_id)));
    }

    #[test]
    fn ingest_rejected_for_unknown_source() {
        let mut hub = IntegrationHub::new();
        assert!(!hub.ingest(make_record(EntityId::new())));
    }

    #[test]
    fn ring_buffer_eviction() {
        let mut hub = IntegrationHub::with_max_records(3);
        let src = make_source(DataSourceKind::AirQualityNetwork);
        let src_id = src.id;
        hub.register_source(src);

        for _ in 0..5 {
            hub.ingest(make_record(src_id));
        }

        let recent = hub.recent_records(&src_id, 10);
        assert_eq!(recent.len(), 3); // Only last 3 retained.
        assert_eq!(hub.statistics(&src_id).unwrap().total_records, 5);
    }

    #[test]
    fn health_healthy_after_recent_data() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::TrafficManagementCentre);
        let src_id = src.id;
        hub.register_source(src);
        hub.ingest(make_record(src_id));

        assert_eq!(hub.assess_health(&src_id), SourceHealth::Healthy);
    }

    #[test]
    fn health_stale_when_no_data() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::ParkingOperator);
        let src_id = src.id;
        hub.register_source(src);

        // No data ingested → stale.
        assert_eq!(hub.assess_health(&src_id), SourceHealth::Stale);
    }

    #[test]
    fn health_offline_when_disabled() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::ChargingNetwork);
        let src_id = src.id;
        hub.register_source(src);
        hub.disable_source(&src_id);

        assert_eq!(hub.assess_health(&src_id), SourceHealth::Offline);
    }

    #[test]
    fn sources_by_kind() {
        let mut hub = IntegrationHub::new();
        hub.register_source(make_source(DataSourceKind::MunicipalApi));
        hub.register_source(make_source(DataSourceKind::MunicipalApi));
        hub.register_source(make_source(DataSourceKind::WeatherService));

        assert_eq!(hub.sources_by_kind(DataSourceKind::MunicipalApi).len(), 2);
        assert_eq!(hub.sources_by_kind(DataSourceKind::WeatherService).len(), 1);
    }

    #[test]
    fn record_error_increments() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::EmergencyServicesFeed);
        let src_id = src.id;
        hub.register_source(src);

        hub.record_error(&src_id);
        hub.record_error(&src_id);

        let stats = hub.statistics(&src_id).unwrap();
        assert_eq!(stats.error_count, 2);
    }

    #[test]
    fn latency_tracking() {
        let mut hub = IntegrationHub::new();
        let src = make_source(DataSourceKind::PublicTransportFeed);
        let src_id = src.id;
        hub.register_source(src);

        hub.ingest(make_record(src_id));
        let stats = hub.statistics(&src_id).unwrap();
        assert!(stats.avg_latency_ms >= 0.0);
    }
}
