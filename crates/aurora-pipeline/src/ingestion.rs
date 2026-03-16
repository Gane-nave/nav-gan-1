//! Data ingestion — sources, connectors, and record parsing.

use std::collections::HashMap;

/// Supported data source types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    /// File-based input (CSV, JSON, Parquet).
    File,
    /// HTTP/REST API endpoint.
    HttpApi,
    /// Message queue (Kafka, AMQP).
    MessageQueue,
    /// Database query.
    Database,
    /// Real-time sensor stream.
    Sensor,
}

/// Data format for ingested records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Json,
    Csv,
    Binary,
    Protobuf,
    Avro,
}

/// A single ingested record.
#[derive(Debug, Clone)]
pub struct Record {
    /// Unique record identifier.
    pub id: String,
    /// Source that produced this record.
    pub source_id: String,
    /// Timestamp of ingestion (epoch millis).
    pub ingested_at_ms: u64,
    /// Key-value payload.
    pub fields: HashMap<String, String>,
    /// Raw byte size.
    pub byte_size: usize,
}

/// Ingestion source configuration.
#[derive(Debug, Clone)]
pub struct SourceConfig {
    /// Source identifier.
    pub id: String,
    /// Source type.
    pub source_type: SourceType,
    /// Data format.
    pub format: DataFormat,
    /// Whether the source is active.
    pub active: bool,
    /// Maximum records per batch.
    pub batch_size: usize,
    /// Connection string or URI.
    pub uri: String,
}

/// Ingestion engine — manages data sources and record collection.
pub struct IngestionEngine {
    sources: HashMap<String, SourceConfig>,
    records: Vec<Record>,
    total_bytes_ingested: u64,
    total_records_ingested: u64,
    errors: Vec<IngestionError>,
}

/// Ingestion error.
#[derive(Debug, Clone)]
pub struct IngestionError {
    /// Source that produced the error.
    pub source_id: String,
    /// Error message.
    pub message: String,
    /// Timestamp (epoch millis).
    pub timestamp_ms: u64,
}

impl IngestionEngine {
    /// Create a new ingestion engine.
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            records: Vec::new(),
            total_bytes_ingested: 0,
            total_records_ingested: 0,
            errors: Vec::new(),
        }
    }

    /// Register a data source.
    pub fn register_source(&mut self, config: SourceConfig) -> Result<(), String> {
        if self.sources.contains_key(&config.id) {
            return Err(format!("Source '{}' already registered", config.id));
        }
        self.sources.insert(config.id.clone(), config);
        Ok(())
    }

    /// Unregister a data source.
    pub fn unregister_source(&mut self, id: &str) -> Result<SourceConfig, String> {
        self.sources
            .remove(id)
            .ok_or_else(|| format!("Source '{id}' not found"))
    }

    /// Ingest a batch of records from a source.
    pub fn ingest(&mut self, source_id: &str, records: Vec<Record>) -> Result<usize, String> {
        let source = self
            .sources
            .get(source_id)
            .ok_or_else(|| format!("Source '{source_id}' not found"))?;
        if !source.active {
            return Err(format!("Source '{source_id}' is not active"));
        }
        if records.len() > source.batch_size {
            return Err(format!(
                "Batch size {} exceeds max {} for source '{source_id}'",
                records.len(),
                source.batch_size
            ));
        }
        let count = records.len();
        let bytes: usize = records.iter().map(|r| r.byte_size).sum();
        self.total_bytes_ingested += bytes as u64;
        self.total_records_ingested += count as u64;
        self.records.extend(records);
        Ok(count)
    }

    /// Record an ingestion error.
    pub fn record_error(&mut self, source_id: &str, message: &str, timestamp_ms: u64) {
        self.errors.push(IngestionError {
            source_id: source_id.to_string(),
            message: message.to_string(),
            timestamp_ms,
        });
    }

    /// Get all pending records (drains the buffer).
    pub fn drain_records(&mut self) -> Vec<Record> {
        std::mem::take(&mut self.records)
    }

    /// Get total bytes ingested.
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes_ingested
    }

    /// Get total records ingested.
    pub fn total_records(&self) -> u64 {
        self.total_records_ingested
    }

    /// Get error count.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get source count.
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    /// Get active source count.
    pub fn active_sources(&self) -> usize {
        self.sources.values().filter(|s| s.active).count()
    }

    /// Get buffered record count.
    pub fn buffered_records(&self) -> usize {
        self.records.len()
    }

    /// Get a source by ID.
    pub fn get_source(&self, id: &str) -> Option<&SourceConfig> {
        self.sources.get(id)
    }

    /// Set source active/inactive.
    pub fn set_source_active(&mut self, id: &str, active: bool) -> Result<(), String> {
        let source = self
            .sources
            .get_mut(id)
            .ok_or_else(|| format!("Source '{id}' not found"))?;
        source.active = active;
        Ok(())
    }
}

impl Default for IngestionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_source(id: &str, active: bool) -> SourceConfig {
        SourceConfig {
            id: id.to_string(),
            source_type: SourceType::HttpApi,
            format: DataFormat::Json,
            active,
            batch_size: 100,
            uri: "http://localhost".to_string(),
        }
    }

    fn make_record(id: &str, source_id: &str, bytes: usize) -> Record {
        Record {
            id: id.to_string(),
            source_id: source_id.to_string(),
            ingested_at_ms: 1000,
            fields: HashMap::new(),
            byte_size: bytes,
        }
    }

    #[test]
    fn test_register_and_ingest() {
        let mut engine = IngestionEngine::new();
        engine.register_source(make_source("s1", true)).unwrap();
        let records = vec![make_record("r1", "s1", 100), make_record("r2", "s1", 200)];
        let count = engine.ingest("s1", records).unwrap();
        assert_eq!(count, 2);
        assert_eq!(engine.total_records(), 2);
        assert_eq!(engine.total_bytes(), 300);
        assert_eq!(engine.buffered_records(), 2);
    }

    #[test]
    fn test_ingest_inactive_source_rejected() {
        let mut engine = IngestionEngine::new();
        engine.register_source(make_source("s1", false)).unwrap();
        let err = engine
            .ingest("s1", vec![make_record("r1", "s1", 50)])
            .unwrap_err();
        assert!(err.contains("not active"));
    }

    #[test]
    fn test_ingest_unknown_source_rejected() {
        let mut engine = IngestionEngine::new();
        let err = engine
            .ingest("missing", vec![make_record("r1", "missing", 50)])
            .unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_batch_size_exceeded() {
        let mut engine = IngestionEngine::new();
        let mut src = make_source("s1", true);
        src.batch_size = 1;
        engine.register_source(src).unwrap();
        let err = engine
            .ingest(
                "s1",
                vec![make_record("r1", "s1", 50), make_record("r2", "s1", 50)],
            )
            .unwrap_err();
        assert!(err.contains("exceeds max"));
    }

    #[test]
    fn test_drain_records() {
        let mut engine = IngestionEngine::new();
        engine.register_source(make_source("s1", true)).unwrap();
        engine
            .ingest("s1", vec![make_record("r1", "s1", 100)])
            .unwrap();
        let drained = engine.drain_records();
        assert_eq!(drained.len(), 1);
        assert_eq!(engine.buffered_records(), 0);
        // total counters not affected by drain
        assert_eq!(engine.total_records(), 1);
    }

    #[test]
    fn test_register_duplicate_source_rejected() {
        let mut engine = IngestionEngine::new();
        engine.register_source(make_source("s1", true)).unwrap();
        let err = engine.register_source(make_source("s1", true)).unwrap_err();
        assert!(err.contains("already registered"));
    }

    #[test]
    fn test_unregister_source() {
        let mut engine = IngestionEngine::new();
        engine.register_source(make_source("s1", true)).unwrap();
        let removed = engine.unregister_source("s1").unwrap();
        assert_eq!(removed.id, "s1");
        assert_eq!(engine.source_count(), 0);
    }

    #[test]
    fn test_set_source_active() {
        let mut engine = IngestionEngine::new();
        engine.register_source(make_source("s1", true)).unwrap();
        engine.set_source_active("s1", false).unwrap();
        assert!(!engine.get_source("s1").unwrap().active);
        let err = engine
            .ingest("s1", vec![make_record("r1", "s1", 50)])
            .unwrap_err();
        assert!(err.contains("not active"));
    }

    #[test]
    fn test_error_recording() {
        let mut engine = IngestionEngine::new();
        engine.record_error("s1", "connection timeout", 1000);
        engine.record_error("s1", "parse error", 2000);
        assert_eq!(engine.error_count(), 2);
    }

    #[test]
    fn test_active_sources_count() {
        let mut engine = IngestionEngine::new();
        engine.register_source(make_source("s1", true)).unwrap();
        engine.register_source(make_source("s2", false)).unwrap();
        engine.register_source(make_source("s3", true)).unwrap();
        assert_eq!(engine.active_sources(), 2);
        assert_eq!(engine.source_count(), 3);
    }
}
