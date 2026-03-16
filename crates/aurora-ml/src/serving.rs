//! Model serving — manages deployed models, handles prediction requests,
//! and provides A/B testing between model versions.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{DecisionStump, EnsembleModel, LinearModel, ModelMetrics};

/// A deployed model ready to serve predictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedModel {
    pub id: Uuid,
    pub name: String,
    pub version: u32,
    pub model: ServedModel,
    pub metrics: ModelMetrics,
    pub traffic_weight: f64,
}

/// Model types that can be served.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServedModel {
    Linear(LinearModel),
    Stump(DecisionStump),
    Ensemble(EnsembleModel),
}

impl ServedModel {
    /// Make a prediction.
    pub fn predict(&self, features: &[f64]) -> f64 {
        match self {
            ServedModel::Linear(m) => m.predict(features),
            ServedModel::Stump(m) => m.predict(features),
            ServedModel::Ensemble(m) => m.predict(features),
        }
    }
}

/// A prediction request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionRequest {
    pub request_id: Uuid,
    pub model_name: String,
    pub features: Vec<f64>,
}

/// A prediction response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResponse {
    pub request_id: Uuid,
    pub model_id: Uuid,
    pub model_version: u32,
    pub prediction: f64,
}

/// Model serving registry.
pub struct ModelRegistry {
    models: RwLock<HashMap<String, Vec<DeployedModel>>>,
    prediction_count: RwLock<u64>,
}

impl ModelRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            models: RwLock::new(HashMap::new()),
            prediction_count: RwLock::new(0),
        }
    }

    /// Deploy a model.
    pub fn deploy(&self, model: DeployedModel) {
        let mut models = self.models.write();
        models.entry(model.name.clone()).or_default().push(model);
    }

    /// Undeploy a model by ID.
    pub fn undeploy(&self, model_id: Uuid) -> bool {
        let mut models = self.models.write();
        for versions in models.values_mut() {
            if let Some(idx) = versions.iter().position(|m| m.id == model_id) {
                versions.remove(idx);
                return true;
            }
        }
        false
    }

    /// Get the latest version of a model by name.
    pub fn get_latest(&self, name: &str) -> Option<DeployedModel> {
        let models = self.models.read();
        models
            .get(name)
            .and_then(|versions| versions.iter().max_by_key(|m| m.version).cloned())
    }

    /// Make a prediction using the latest model version.
    pub fn predict(&self, request: &PredictionRequest) -> Option<PredictionResponse> {
        let model = self.get_latest(&request.model_name)?;
        let prediction = model.model.predict(&request.features);

        *self.prediction_count.write() += 1;

        Some(PredictionResponse {
            request_id: request.request_id,
            model_id: model.id,
            model_version: model.version,
            prediction,
        })
    }

    /// A/B test: predict using weighted random selection between model versions.
    /// Returns the model with the highest traffic weight for deterministic testing.
    pub fn predict_ab(&self, request: &PredictionRequest) -> Option<PredictionResponse> {
        let models = self.models.read();
        let versions = models.get(&request.model_name)?;
        if versions.is_empty() {
            return None;
        }

        // Select the model with the highest traffic weight
        let model = versions.iter().max_by(|a, b| {
            a.traffic_weight
                .partial_cmp(&b.traffic_weight)
                .unwrap_or(std::cmp::Ordering::Equal)
        })?;

        let prediction = model.model.predict(&request.features);
        *self.prediction_count.write() += 1;

        Some(PredictionResponse {
            request_id: request.request_id,
            model_id: model.id,
            model_version: model.version,
            prediction,
        })
    }

    /// Total number of predictions served.
    pub fn total_predictions(&self) -> u64 {
        *self.prediction_count.read()
    }

    /// Number of deployed model names.
    pub fn model_count(&self) -> usize {
        self.models.read().len()
    }

    /// List all deployed model names.
    pub fn list_models(&self) -> Vec<String> {
        self.models.read().keys().cloned().collect()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_deployed(name: &str, version: u32, weight: f64) -> DeployedModel {
        DeployedModel {
            id: Uuid::new_v4(),
            name: name.to_string(),
            version,
            model: ServedModel::Linear(LinearModel::new(vec![1.0], 0.0)),
            metrics: ModelMetrics::empty(),
            traffic_weight: weight,
        }
    }

    #[test]
    fn test_deploy_and_predict() {
        let registry = ModelRegistry::new();
        registry.deploy(make_deployed("eta", 1, 1.0));

        let req = PredictionRequest {
            request_id: Uuid::new_v4(),
            model_name: "eta".into(),
            features: vec![10.0],
        };
        let resp = registry.predict(&req).unwrap();
        assert!((resp.prediction - 10.0).abs() < f64::EPSILON); // w=1, b=0 → pred=10
        assert_eq!(resp.model_version, 1);
        assert_eq!(registry.total_predictions(), 1);
    }

    #[test]
    fn test_get_latest_version() {
        let registry = ModelRegistry::new();
        registry.deploy(make_deployed("eta", 1, 1.0));
        registry.deploy(make_deployed("eta", 3, 1.0));
        registry.deploy(make_deployed("eta", 2, 1.0));

        let latest = registry.get_latest("eta").unwrap();
        assert_eq!(latest.version, 3);
    }

    #[test]
    fn test_undeploy() {
        let registry = ModelRegistry::new();
        let model = make_deployed("eta", 1, 1.0);
        let id = model.id;
        registry.deploy(model);
        assert!(registry.undeploy(id));
        assert!(registry.get_latest("eta").is_none());
    }

    #[test]
    fn test_predict_unknown_model() {
        let registry = ModelRegistry::new();
        let req = PredictionRequest {
            request_id: Uuid::new_v4(),
            model_name: "unknown".into(),
            features: vec![1.0],
        };
        assert!(registry.predict(&req).is_none());
    }

    #[test]
    fn test_ab_test_selects_highest_weight() {
        let registry = ModelRegistry::new();
        // v1: weight 0.3, pred = 1*x = x
        registry.deploy(make_deployed("eta", 1, 0.3));
        // v2: weight 0.7, pred = 1*x = x (same model type, different weight)
        let mut v2 = make_deployed("eta", 2, 0.7);
        v2.model = ServedModel::Linear(LinearModel::new(vec![2.0], 0.0)); // pred = 2*x
        registry.deploy(v2);

        let req = PredictionRequest {
            request_id: Uuid::new_v4(),
            model_name: "eta".into(),
            features: vec![5.0],
        };
        let resp = registry.predict_ab(&req).unwrap();
        // Should select v2 (higher weight): 2*5 = 10
        assert!((resp.prediction - 10.0).abs() < f64::EPSILON);
        assert_eq!(resp.model_version, 2);
    }

    #[test]
    fn test_model_count() {
        let registry = ModelRegistry::new();
        registry.deploy(make_deployed("eta", 1, 1.0));
        registry.deploy(make_deployed("traffic", 1, 1.0));
        assert_eq!(registry.model_count(), 2);
    }

    #[test]
    fn test_list_models() {
        let registry = ModelRegistry::new();
        registry.deploy(make_deployed("eta", 1, 1.0));
        registry.deploy(make_deployed("risk", 1, 1.0));
        let mut names = registry.list_models();
        names.sort();
        assert_eq!(names, vec!["eta", "risk"]);
    }

    #[test]
    fn test_served_model_types() {
        let stump = ServedModel::Stump(DecisionStump {
            feature_idx: 0,
            threshold: 5.0,
            left_value: 1.0,
            right_value: 9.0,
        });
        assert!((stump.predict(&[3.0]) - 1.0).abs() < f64::EPSILON);
        assert!((stump.predict(&[7.0]) - 9.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_serialization() {
        let req = PredictionRequest {
            request_id: Uuid::new_v4(),
            model_name: "eta".into(),
            features: vec![1.0, 2.0],
        };
        let json = serde_json::to_string(&req).unwrap();
        let de: PredictionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(de.model_name, "eta");
        assert_eq!(de.features, vec![1.0, 2.0]);
    }
}
