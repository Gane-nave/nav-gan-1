//! Edge inference — on-device ML model management and inference.
//!
//! Manages model versions, runs inference locally, and handles
//! model updates from the cloud.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Type of ML model deployed at the edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelType {
    /// Road surface classification.
    RoadSurface,
    /// Traffic sign detection.
    TrafficSign,
    /// Lane marking detection.
    LaneMarking,
    /// Driving behavior classification.
    DrivingBehavior,
    /// Weather condition estimation.
    WeatherEstimation,
    /// Anomaly detection (sensor faults).
    AnomalyDetection,
}

/// Status of a deployed model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    /// Model is loading into memory.
    Loading,
    /// Model is ready for inference.
    Ready,
    /// Model update available from cloud.
    UpdateAvailable,
    /// Model is currently running inference.
    Busy,
    /// Model failed to load.
    Failed,
    /// Model has been unloaded to save memory.
    Unloaded,
}

/// A deployed edge model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeModel {
    pub id: EntityId,
    pub model_type: ModelType,
    pub version: String,
    pub size_bytes: u64,
    pub status: ModelStatus,
    pub loaded_at: Option<DateTime<Utc>>,
    pub last_inference: Option<DateTime<Utc>>,
    pub total_inferences: u64,
    pub avg_latency_ms: f64,
    pub accuracy_score: f64,
}

/// Result of an inference operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    pub model_id: EntityId,
    pub model_type: ModelType,
    pub prediction: String,
    pub confidence: f64,
    pub latency_ms: f64,
    pub timestamp: DateTime<Utc>,
}

/// Edge inference engine — manages models and runs predictions.
pub struct InferenceEngine {
    models: Vec<EdgeModel>,
    /// Maximum memory budget for models (bytes).
    memory_budget_bytes: u64,
    /// Current memory used by loaded models.
    memory_used_bytes: u64,
    /// Inference history.
    history: Vec<InferenceResult>,
    max_history: usize,
}

impl InferenceEngine {
    pub fn new(memory_budget_bytes: u64) -> Self {
        Self {
            models: Vec::new(),
            memory_budget_bytes,
            memory_used_bytes: 0,
            history: Vec::new(),
            max_history: 500,
        }
    }

    /// Register a model for deployment.
    pub fn register_model(
        &mut self,
        model_type: ModelType,
        version: &str,
        size_bytes: u64,
    ) -> EntityId {
        let id = EntityId::new();
        let model = EdgeModel {
            id,
            model_type,
            version: version.to_string(),
            size_bytes,
            status: ModelStatus::Loading,
            loaded_at: None,
            last_inference: None,
            total_inferences: 0,
            avg_latency_ms: 0.0,
            accuracy_score: 1.0,
        };
        info!(
            model_type = ?model_type,
            version = version,
            size = size_bytes,
            "registered edge model"
        );
        self.models.push(model);
        id
    }

    /// Load a model into memory. Returns false if insufficient budget.
    pub fn load_model(&mut self, model_id: &EntityId) -> bool {
        let Some(model) = self.models.iter_mut().find(|m| m.id == *model_id) else {
            return false;
        };

        if model.status == ModelStatus::Ready {
            return true; // Already loaded.
        }

        if self.memory_used_bytes + model.size_bytes > self.memory_budget_bytes {
            warn!(
                model = %model_id,
                needed = model.size_bytes,
                available = self.memory_budget_bytes - self.memory_used_bytes,
                "insufficient memory budget for model"
            );
            model.status = ModelStatus::Failed;
            return false;
        }

        self.memory_used_bytes += model.size_bytes;
        model.status = ModelStatus::Ready;
        model.loaded_at = Some(Utc::now());
        debug!(model = %model_id, "model loaded");
        true
    }

    /// Unload a model to free memory.
    pub fn unload_model(&mut self, model_id: &EntityId) -> bool {
        let Some(model) = self.models.iter_mut().find(|m| m.id == *model_id) else {
            return false;
        };

        if model.status != ModelStatus::Ready && model.status != ModelStatus::Busy {
            return false;
        }

        self.memory_used_bytes = self.memory_used_bytes.saturating_sub(model.size_bytes);
        model.status = ModelStatus::Unloaded;
        info!(model = %model_id, "model unloaded");
        true
    }

    /// Run inference on a loaded model. Returns None if model is not ready.
    pub fn infer(
        &mut self,
        model_id: &EntityId,
        prediction: &str,
        confidence: f64,
        latency_ms: f64,
    ) -> Option<InferenceResult> {
        let model = self.models.iter_mut().find(|m| m.id == *model_id)?;

        if model.status != ModelStatus::Ready {
            warn!(
                model = %model_id,
                status = ?model.status,
                "cannot infer — model not ready"
            );
            return None;
        }

        model.total_inferences += 1;
        let n = model.total_inferences as f64;
        model.avg_latency_ms = model.avg_latency_ms * ((n - 1.0) / n) + latency_ms / n;
        model.last_inference = Some(Utc::now());

        let result = InferenceResult {
            model_id: *model_id,
            model_type: model.model_type,
            prediction: prediction.to_string(),
            confidence,
            latency_ms,
            timestamp: Utc::now(),
        };

        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push(result.clone());

        Some(result)
    }

    /// Update a model version (marks as UpdateAvailable; must reload).
    pub fn mark_update_available(&mut self, model_id: &EntityId, new_version: &str) -> bool {
        if let Some(model) = self.models.iter_mut().find(|m| m.id == *model_id) {
            model.version = new_version.to_string();
            model.status = ModelStatus::UpdateAvailable;
            info!(
                model = %model_id,
                version = new_version,
                "model update available"
            );
            true
        } else {
            false
        }
    }

    /// Get all models of a given type.
    pub fn models_by_type(&self, model_type: ModelType) -> Vec<&EdgeModel> {
        self.models
            .iter()
            .filter(|m| m.model_type == model_type)
            .collect()
    }

    /// Get a model by ID.
    pub fn model(&self, model_id: &EntityId) -> Option<&EdgeModel> {
        self.models.iter().find(|m| m.id == *model_id)
    }

    /// Get all loaded (ready) models.
    pub fn loaded_models(&self) -> Vec<&EdgeModel> {
        self.models
            .iter()
            .filter(|m| m.status == ModelStatus::Ready)
            .collect()
    }

    /// Memory used by loaded models.
    pub fn memory_used(&self) -> u64 {
        self.memory_used_bytes
    }

    /// Memory budget.
    pub fn memory_budget(&self) -> u64 {
        self.memory_budget_bytes
    }

    /// Memory available.
    pub fn memory_available(&self) -> u64 {
        self.memory_budget_bytes
            .saturating_sub(self.memory_used_bytes)
    }

    /// Number of registered models.
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Inference history.
    pub fn history(&self) -> &[InferenceResult] {
        &self.history
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new(256 * 1024 * 1024) // 256 MB default budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_load_model() {
        let mut engine = InferenceEngine::new(100_000_000);
        let id = engine.register_model(ModelType::RoadSurface, "1.0.0", 10_000_000);

        assert!(engine.load_model(&id));
        let model = engine.model(&id).unwrap();
        assert_eq!(model.status, ModelStatus::Ready);
        assert_eq!(engine.memory_used(), 10_000_000);
    }

    #[test]
    fn load_exceeds_budget_fails() {
        let mut engine = InferenceEngine::new(5_000_000);
        let id = engine.register_model(ModelType::LaneMarking, "1.0.0", 10_000_000);

        assert!(!engine.load_model(&id));
        let model = engine.model(&id).unwrap();
        assert_eq!(model.status, ModelStatus::Failed);
    }

    #[test]
    fn unload_frees_memory() {
        let mut engine = InferenceEngine::new(100_000_000);
        let id = engine.register_model(ModelType::TrafficSign, "2.0", 20_000_000);
        engine.load_model(&id);

        assert_eq!(engine.memory_used(), 20_000_000);
        assert!(engine.unload_model(&id));
        assert_eq!(engine.memory_used(), 0);
    }

    #[test]
    fn inference_on_ready_model() {
        let mut engine = InferenceEngine::new(100_000_000);
        let id = engine.register_model(ModelType::DrivingBehavior, "1.0", 5_000_000);
        engine.load_model(&id);

        let result = engine.infer(&id, "aggressive", 0.85, 12.5).unwrap();
        assert_eq!(result.prediction, "aggressive");
        assert!((result.confidence - 0.85).abs() < f64::EPSILON);

        let model = engine.model(&id).unwrap();
        assert_eq!(model.total_inferences, 1);
    }

    #[test]
    fn inference_on_unloaded_model_fails() {
        let mut engine = InferenceEngine::new(100_000_000);
        let id = engine.register_model(ModelType::WeatherEstimation, "1.0", 5_000_000);
        // Not loaded.
        assert!(engine.infer(&id, "rain", 0.9, 10.0).is_none());
    }

    #[test]
    fn mark_update_available() {
        let mut engine = InferenceEngine::new(100_000_000);
        let id = engine.register_model(ModelType::AnomalyDetection, "1.0", 5_000_000);
        engine.load_model(&id);

        assert!(engine.mark_update_available(&id, "2.0"));
        let model = engine.model(&id).unwrap();
        assert_eq!(model.status, ModelStatus::UpdateAvailable);
        assert_eq!(model.version, "2.0");
    }

    #[test]
    fn models_by_type_filter() {
        let mut engine = InferenceEngine::new(100_000_000);
        engine.register_model(ModelType::RoadSurface, "1.0", 1000);
        engine.register_model(ModelType::RoadSurface, "2.0", 2000);
        engine.register_model(ModelType::TrafficSign, "1.0", 1000);

        assert_eq!(engine.models_by_type(ModelType::RoadSurface).len(), 2);
        assert_eq!(engine.models_by_type(ModelType::TrafficSign).len(), 1);
        assert_eq!(engine.models_by_type(ModelType::LaneMarking).len(), 0);
    }

    #[test]
    fn loaded_models_filter() {
        let mut engine = InferenceEngine::new(100_000_000);
        let id1 = engine.register_model(ModelType::RoadSurface, "1.0", 1000);
        let _id2 = engine.register_model(ModelType::TrafficSign, "1.0", 1000);
        engine.load_model(&id1);

        assert_eq!(engine.loaded_models().len(), 1);
    }

    #[test]
    fn avg_latency_updated() {
        let mut engine = InferenceEngine::new(100_000_000);
        let id = engine.register_model(ModelType::RoadSurface, "1.0", 1000);
        engine.load_model(&id);

        engine.infer(&id, "asphalt", 0.9, 10.0);
        engine.infer(&id, "gravel", 0.8, 20.0);

        let model = engine.model(&id).unwrap();
        assert!((model.avg_latency_ms - 15.0).abs() < 0.01);
    }

    #[test]
    fn memory_available_calculated() {
        let mut engine = InferenceEngine::new(100);
        let id = engine.register_model(ModelType::RoadSurface, "1.0", 30);
        engine.load_model(&id);

        assert_eq!(engine.memory_available(), 70);
    }

    #[test]
    fn load_already_ready_is_idempotent() {
        let mut engine = InferenceEngine::new(100_000_000);
        let id = engine.register_model(ModelType::RoadSurface, "1.0", 5000);
        engine.load_model(&id);
        assert!(engine.load_model(&id)); // Should return true, not double-count memory.
        assert_eq!(engine.memory_used(), 5000);
    }
}
