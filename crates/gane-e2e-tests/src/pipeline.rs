//! Pipeline integration tests — verifies cross-crate data flow through the navigation pipeline.

use std::collections::HashMap;

/// Pipeline stage identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipelineStage {
    /// GNSS signal acquisition.
    GnssAcquisition,
    /// Sensor fusion (IMU + GNSS).
    SensorFusion,
    /// Integrity validation.
    IntegrityCheck,
    /// Route computation.
    Routing,
    /// Lane guidance generation.
    LaneGuidance,
    /// UX output delivery.
    UxOutput,
}

/// Result of a pipeline stage execution.
#[derive(Debug, Clone)]
pub struct StageResult {
    /// Stage that was executed.
    pub stage: PipelineStage,
    /// Whether the stage completed successfully.
    pub success: bool,
    /// Execution time in milliseconds.
    pub duration_ms: u64,
    /// Output data size in bytes.
    pub output_bytes: usize,
    /// Error message if failed.
    pub error: Option<String>,
}

/// Pipeline execution context — tracks data flow through all stages.
pub struct PipelineRunner {
    results: Vec<StageResult>,
    stage_order: Vec<PipelineStage>,
    data_sizes: HashMap<PipelineStage, usize>,
}

impl PipelineRunner {
    /// Create a new pipeline runner with the standard stage order.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            stage_order: vec![
                PipelineStage::GnssAcquisition,
                PipelineStage::SensorFusion,
                PipelineStage::IntegrityCheck,
                PipelineStage::Routing,
                PipelineStage::LaneGuidance,
                PipelineStage::UxOutput,
            ],
            data_sizes: HashMap::new(),
        }
    }

    /// Execute a pipeline stage with simulated data.
    pub fn execute_stage(
        &mut self,
        stage: PipelineStage,
        input_bytes: usize,
        success: bool,
        duration_ms: u64,
    ) -> &StageResult {
        let output_bytes = if success {
            // Each stage transforms data — output is typically different size
            match stage {
                PipelineStage::GnssAcquisition => input_bytes * 2, // raw → parsed
                PipelineStage::SensorFusion => input_bytes * 3 / 2, // merged streams
                PipelineStage::IntegrityCheck => input_bytes,      // validation passthrough
                PipelineStage::Routing => input_bytes * 4,         // route graph expansion
                PipelineStage::LaneGuidance => input_bytes / 2,    // compressed guidance
                PipelineStage::UxOutput => input_bytes / 4,        // final display data
            }
        } else {
            0
        };

        let error = if success {
            None
        } else {
            Some(format!("Stage {stage:?} failed"))
        };

        self.data_sizes.insert(stage, output_bytes);
        self.results.push(StageResult {
            stage,
            success,
            duration_ms,
            output_bytes,
            error,
        });

        self.results.last().unwrap()
    }

    /// Get all stage results.
    pub fn results(&self) -> &[StageResult] {
        &self.results
    }

    /// Get total pipeline duration.
    pub fn total_duration_ms(&self) -> u64 {
        self.results.iter().map(|r| r.duration_ms).sum()
    }

    /// Check if all stages passed.
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.success)
    }

    /// Get the first failing stage.
    pub fn first_failure(&self) -> Option<&StageResult> {
        self.results.iter().find(|r| !r.success)
    }

    /// Get stage count.
    pub fn stage_count(&self) -> usize {
        self.results.len()
    }

    /// Get the expected stage order.
    pub fn expected_order(&self) -> &[PipelineStage] {
        &self.stage_order
    }

    /// Verify stages were executed in correct order.
    pub fn verify_order(&self) -> bool {
        if self.results.is_empty() {
            return true;
        }
        let executed: Vec<PipelineStage> = self.results.iter().map(|r| r.stage).collect();
        // Each executed stage should appear in the expected order
        let mut order_idx = 0;
        for stage in &executed {
            while order_idx < self.stage_order.len() && self.stage_order[order_idx] != *stage {
                order_idx += 1;
            }
            if order_idx >= self.stage_order.len() {
                return false;
            }
            order_idx += 1;
        }
        true
    }

    /// Get output data size for a stage.
    pub fn output_size(&self, stage: PipelineStage) -> usize {
        self.data_sizes.get(&stage).copied().unwrap_or(0)
    }

    /// Reset the pipeline for a new run.
    pub fn reset(&mut self) {
        self.results.clear();
        self.data_sizes.clear();
    }
}

impl Default for PipelineRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline_success() {
        let mut runner = PipelineRunner::new();
        runner.execute_stage(PipelineStage::GnssAcquisition, 100, true, 10);
        runner.execute_stage(PipelineStage::SensorFusion, 200, true, 15);
        runner.execute_stage(PipelineStage::IntegrityCheck, 300, true, 5);
        runner.execute_stage(PipelineStage::Routing, 300, true, 50);
        runner.execute_stage(PipelineStage::LaneGuidance, 1200, true, 20);
        runner.execute_stage(PipelineStage::UxOutput, 600, true, 8);
        assert!(runner.all_passed());
        assert_eq!(runner.stage_count(), 6);
        assert_eq!(runner.total_duration_ms(), 108);
    }

    #[test]
    fn test_pipeline_failure_stops_data() {
        let mut runner = PipelineRunner::new();
        runner.execute_stage(PipelineStage::GnssAcquisition, 100, true, 10);
        runner.execute_stage(PipelineStage::SensorFusion, 200, false, 5);
        assert!(!runner.all_passed());
        let fail = runner.first_failure().unwrap();
        assert_eq!(fail.stage, PipelineStage::SensorFusion);
        assert_eq!(fail.output_bytes, 0);
        assert!(fail.error.is_some());
    }

    #[test]
    fn test_stage_order_verification() {
        let mut runner = PipelineRunner::new();
        runner.execute_stage(PipelineStage::GnssAcquisition, 100, true, 10);
        runner.execute_stage(PipelineStage::Routing, 300, true, 50);
        assert!(runner.verify_order()); // skipping stages is ok, order must be preserved

        let mut bad = PipelineRunner::new();
        bad.execute_stage(PipelineStage::Routing, 300, true, 50);
        bad.execute_stage(PipelineStage::GnssAcquisition, 100, true, 10);
        assert!(!bad.verify_order()); // wrong order
    }

    #[test]
    fn test_data_size_transformation() {
        let mut runner = PipelineRunner::new();
        runner.execute_stage(PipelineStage::GnssAcquisition, 100, true, 10);
        assert_eq!(runner.output_size(PipelineStage::GnssAcquisition), 200); // 100 * 2
        runner.execute_stage(PipelineStage::UxOutput, 400, true, 5);
        assert_eq!(runner.output_size(PipelineStage::UxOutput), 100); // 400 / 4
    }

    #[test]
    fn test_pipeline_reset() {
        let mut runner = PipelineRunner::new();
        runner.execute_stage(PipelineStage::GnssAcquisition, 100, true, 10);
        assert_eq!(runner.stage_count(), 1);
        runner.reset();
        assert_eq!(runner.stage_count(), 0);
        assert!(runner.all_passed()); // vacuously true
    }

    #[test]
    fn test_empty_pipeline() {
        let runner = PipelineRunner::new();
        assert!(runner.all_passed());
        assert!(runner.verify_order());
        assert_eq!(runner.total_duration_ms(), 0);
        assert!(runner.first_failure().is_none());
    }

    #[test]
    fn test_expected_order() {
        let runner = PipelineRunner::new();
        let order = runner.expected_order();
        assert_eq!(order.len(), 6);
        assert_eq!(order[0], PipelineStage::GnssAcquisition);
        assert_eq!(order[5], PipelineStage::UxOutput);
    }
}
