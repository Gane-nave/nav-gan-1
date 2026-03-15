//! Edge processing pipeline — local computation stages for sensor data.
//!
//! Processes raw sensor data on-device before transmission, reducing
//! bandwidth and latency. Stages can be enabled/disabled dynamically.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// A processing stage in the edge pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub id: EntityId,
    pub name: String,
    pub stage_type: StageType,
    pub enabled: bool,
    pub priority: u32,
    /// Average processing time in microseconds.
    pub avg_latency_us: f64,
    /// Total items processed.
    pub items_processed: u64,
    /// Total items dropped (due to overload).
    pub items_dropped: u64,
}

/// Type of processing stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageType {
    /// Raw data ingestion and validation.
    Ingestion,
    /// Noise filtering and smoothing.
    Filter,
    /// Feature extraction (e.g., lane markings, road signs).
    FeatureExtraction,
    /// Data aggregation (combine multiple readings).
    Aggregation,
    /// Anomaly detection on sensor readings.
    AnomalyDetection,
    /// Data compression before transmission.
    Compression,
    /// Final output formatting.
    Output,
}

/// Processing result from a pipeline stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub stage_id: EntityId,
    pub input_size_bytes: u64,
    pub output_size_bytes: u64,
    pub latency_us: f64,
    pub timestamp: DateTime<Utc>,
    pub status: ProcessingStatus,
}

/// Status of a processing operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingStatus {
    Success,
    Filtered,
    Dropped,
    Error,
}

/// Edge processing pipeline — manages ordered processing stages.
pub struct EdgePipeline {
    stages: Vec<PipelineStage>,
    /// Maximum pipeline latency budget in microseconds.
    latency_budget_us: f64,
    /// Total bytes ingested.
    total_ingested: u64,
    /// Total bytes output.
    total_output: u64,
    /// Processing history (recent results).
    history: Vec<ProcessingResult>,
    /// Maximum history size.
    max_history: usize,
}

impl EdgePipeline {
    pub fn new(latency_budget_us: f64) -> Self {
        Self {
            stages: Vec::new(),
            latency_budget_us,
            total_ingested: 0,
            total_output: 0,
            history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Add a processing stage.
    pub fn add_stage(&mut self, name: &str, stage_type: StageType, priority: u32) -> EntityId {
        let id = EntityId::new();
        let stage = PipelineStage {
            id,
            name: name.to_string(),
            stage_type,
            enabled: true,
            priority,
            avg_latency_us: 0.0,
            items_processed: 0,
            items_dropped: 0,
        };
        self.stages.push(stage);
        // Sort by priority (lower = earlier in pipeline).
        self.stages.sort_by_key(|s| s.priority);
        debug!(name = name, stage_type = ?stage_type, "added pipeline stage");
        id
    }

    /// Process data through the pipeline. Returns reduction ratio.
    pub fn process(&mut self, input_size_bytes: u64) -> f64 {
        self.total_ingested += input_size_bytes;
        let mut current_size = input_size_bytes;

        for stage in &mut self.stages {
            if !stage.enabled {
                continue;
            }

            // Simulate processing: each stage may reduce data size.
            let reduction = match stage.stage_type {
                StageType::Ingestion => 1.0,         // No reduction.
                StageType::Filter => 0.9,            // 10% filtered out.
                StageType::FeatureExtraction => 0.7, // 30% reduction to features.
                StageType::Aggregation => 0.5,       // 50% aggregation.
                StageType::AnomalyDetection => 1.0,  // No size change.
                StageType::Compression => 0.4,       // 60% compression.
                StageType::Output => 1.0,            // No change.
            };

            let output_size = (current_size as f64 * reduction) as u64;

            // Simulate latency based on data size.
            let latency = (current_size as f64 * 0.001).max(10.0);

            let result = ProcessingResult {
                stage_id: stage.id,
                input_size_bytes: current_size,
                output_size_bytes: output_size,
                latency_us: latency,
                timestamp: Utc::now(),
                status: ProcessingStatus::Success,
            };

            // Update stage stats.
            stage.items_processed += 1;
            let n = stage.items_processed as f64;
            stage.avg_latency_us = stage.avg_latency_us * ((n - 1.0) / n) + latency / n;

            if self.history.len() >= self.max_history {
                self.history.remove(0);
            }
            self.history.push(result);

            current_size = output_size;
        }

        self.total_output += current_size;

        if input_size_bytes > 0 {
            current_size as f64 / input_size_bytes as f64
        } else {
            1.0
        }
    }

    /// Enable or disable a stage.
    pub fn set_stage_enabled(&mut self, stage_id: &EntityId, enabled: bool) -> bool {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.id == *stage_id) {
            stage.enabled = enabled;
            info!(stage = %stage.name, enabled = enabled, "stage toggled");
            true
        } else {
            false
        }
    }

    /// Get estimated total pipeline latency (sum of avg latencies).
    pub fn estimated_latency_us(&self) -> f64 {
        self.stages
            .iter()
            .filter(|s| s.enabled)
            .map(|s| s.avg_latency_us)
            .sum()
    }

    /// Check if pipeline is within latency budget.
    pub fn within_budget(&self) -> bool {
        self.estimated_latency_us() <= self.latency_budget_us
    }

    /// Get the overall compression ratio (output/input).
    pub fn compression_ratio(&self) -> f64 {
        if self.total_ingested == 0 {
            return 1.0;
        }
        self.total_output as f64 / self.total_ingested as f64
    }

    /// Get all stages.
    pub fn stages(&self) -> &[PipelineStage] {
        &self.stages
    }

    /// Get total bytes ingested.
    pub fn total_ingested(&self) -> u64 {
        self.total_ingested
    }

    /// Get total bytes output.
    pub fn total_output(&self) -> u64 {
        self.total_output
    }

    /// Get processing history.
    pub fn history(&self) -> &[ProcessingResult] {
        &self.history
    }

    /// Number of stages.
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// Latency budget.
    pub fn latency_budget_us(&self) -> f64 {
        self.latency_budget_us
    }
}

impl Default for EdgePipeline {
    fn default() -> Self {
        let mut pipeline = Self::new(5000.0); // 5ms budget
        pipeline.add_stage("ingestion", StageType::Ingestion, 10);
        pipeline.add_stage("noise_filter", StageType::Filter, 20);
        pipeline.add_stage("feature_extract", StageType::FeatureExtraction, 30);
        pipeline.add_stage("aggregator", StageType::Aggregation, 40);
        pipeline.add_stage("anomaly_detect", StageType::AnomalyDetection, 50);
        pipeline.add_stage("compressor", StageType::Compression, 60);
        pipeline.add_stage("output", StageType::Output, 70);
        pipeline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pipeline_has_7_stages() {
        let pipeline = EdgePipeline::default();
        assert_eq!(pipeline.stage_count(), 7);
    }

    #[test]
    fn process_reduces_data_size() {
        let mut pipeline = EdgePipeline::default();
        let ratio = pipeline.process(10_000);
        // Through all stages: 1.0 * 0.9 * 0.7 * 0.5 * 1.0 * 0.4 * 1.0 = 0.126
        assert!(ratio < 0.2);
        assert!(ratio > 0.05);
    }

    #[test]
    fn compression_ratio_tracks() {
        let mut pipeline = EdgePipeline::default();
        pipeline.process(10_000);
        let ratio = pipeline.compression_ratio();
        assert!(ratio < 1.0);
        assert!(ratio > 0.0);
    }

    #[test]
    fn disable_stage_skips_processing() {
        let mut pipeline = EdgePipeline::default();

        // Process with all stages.
        pipeline.process(10_000);
        let ratio_all = pipeline.compression_ratio();

        // Disable compressor and re-process.
        let compressor_id = pipeline
            .stages()
            .iter()
            .find(|s| s.name == "compressor")
            .unwrap()
            .id;
        pipeline.set_stage_enabled(&compressor_id, false);

        // Reset counters for clean comparison.
        let mut pipeline2 = EdgePipeline::default();
        let compressor_id2 = pipeline2
            .stages()
            .iter()
            .find(|s| s.name == "compressor")
            .unwrap()
            .id;
        pipeline2.set_stage_enabled(&compressor_id2, false);
        pipeline2.process(10_000);
        let ratio_no_compress = pipeline2.compression_ratio();

        // Without compression, ratio should be higher (less reduction).
        assert!(ratio_no_compress > ratio_all);
    }

    #[test]
    fn stages_sorted_by_priority() {
        let mut pipeline = EdgePipeline::new(5000.0);
        pipeline.add_stage("last", StageType::Output, 100);
        pipeline.add_stage("first", StageType::Ingestion, 1);
        pipeline.add_stage("middle", StageType::Filter, 50);

        assert_eq!(pipeline.stages()[0].name, "first");
        assert_eq!(pipeline.stages()[1].name, "middle");
        assert_eq!(pipeline.stages()[2].name, "last");
    }

    #[test]
    fn latency_budget_check() {
        let mut pipeline = EdgePipeline::new(1_000_000.0); // 1 second budget
        pipeline.add_stage("fast", StageType::Ingestion, 10);
        pipeline.process(100);
        assert!(pipeline.within_budget());
    }

    #[test]
    fn history_capped_at_max() {
        let mut pipeline = EdgePipeline::new(100_000.0);
        pipeline.add_stage("test", StageType::Ingestion, 10);
        pipeline.max_history = 5;

        for _ in 0..10 {
            pipeline.process(100);
        }
        assert!(pipeline.history().len() <= 5);
    }

    #[test]
    fn empty_input_returns_ratio_1() {
        let mut pipeline = EdgePipeline::default();
        let ratio = pipeline.process(0);
        assert!((ratio - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn total_bytes_tracked() {
        let mut pipeline = EdgePipeline::new(100_000.0);
        pipeline.add_stage("pass", StageType::Ingestion, 10);
        pipeline.process(5000);
        pipeline.process(3000);

        assert_eq!(pipeline.total_ingested(), 8000);
    }

    #[test]
    fn set_stage_enabled_nonexistent_returns_false() {
        let mut pipeline = EdgePipeline::new(1000.0);
        assert!(!pipeline.set_stage_enabled(&EntityId::new(), true));
    }

    #[test]
    fn items_processed_counter() {
        let mut pipeline = EdgePipeline::new(100_000.0);
        pipeline.add_stage("counter_test", StageType::Filter, 10);
        pipeline.process(1000);
        pipeline.process(2000);

        assert_eq!(pipeline.stages()[0].items_processed, 2);
    }
}
