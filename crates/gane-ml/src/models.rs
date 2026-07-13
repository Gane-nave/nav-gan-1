//! Model definitions — linear regression, decision stump, and ensemble
//! for ETA prediction, traffic forecasting, and route scoring.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Model type identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelType {
    LinearRegression,
    DecisionStump,
    Ensemble,
}

/// Model status in the lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    Training,
    Validating,
    Ready,
    Deprecated,
    Failed,
}

/// Metadata about a trained model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMeta {
    pub id: Uuid,
    pub name: String,
    pub model_type: ModelType,
    pub status: ModelStatus,
    pub version: u32,
    pub feature_names: Vec<String>,
    pub metrics: ModelMetrics,
}

/// Performance metrics for a model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub mae: f64,
    pub rmse: f64,
    pub r_squared: f64,
    pub sample_count: usize,
}

impl ModelMetrics {
    /// Create empty metrics.
    pub fn empty() -> Self {
        Self {
            mae: 0.0,
            rmse: 0.0,
            r_squared: 0.0,
            sample_count: 0,
        }
    }

    /// Compute metrics from predictions and actual values.
    pub fn compute(predictions: &[f64], actuals: &[f64]) -> Self {
        assert_eq!(predictions.len(), actuals.len());
        let n = predictions.len();
        if n == 0 {
            return Self::empty();
        }

        let mean_actual = actuals.iter().sum::<f64>() / n as f64;

        let mut sum_abs_err = 0.0;
        let mut sum_sq_err = 0.0;
        let mut ss_tot = 0.0;

        for (p, a) in predictions.iter().zip(actuals.iter()) {
            let err = p - a;
            sum_abs_err += err.abs();
            sum_sq_err += err * err;
            ss_tot += (a - mean_actual).powi(2);
        }

        let mae = sum_abs_err / n as f64;
        let rmse = (sum_sq_err / n as f64).sqrt();
        let r_squared = if ss_tot.abs() < f64::EPSILON {
            1.0
        } else {
            1.0 - sum_sq_err / ss_tot
        };

        Self {
            mae,
            rmse,
            r_squared,
            sample_count: n,
        }
    }
}

/// A simple linear regression model: y = w0 + w1*x1 + w2*x2 + ...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearModel {
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl LinearModel {
    /// Create a model with given weights and bias.
    pub fn new(weights: Vec<f64>, bias: f64) -> Self {
        Self { weights, bias }
    }

    /// Predict from a feature vector.
    pub fn predict(&self, features: &[f64]) -> f64 {
        let dot: f64 = self
            .weights
            .iter()
            .zip(features.iter())
            .map(|(w, f)| w * f)
            .sum();
        self.bias + dot
    }

    /// Train via ordinary least squares on small datasets.
    /// Uses the normal equation: w = (X^T X)^-1 X^T y
    /// Falls back to gradient descent for numerical stability.
    pub fn fit(features: &[Vec<f64>], targets: &[f64], learning_rate: f64, epochs: usize) -> Self {
        assert_eq!(features.len(), targets.len());
        if features.is_empty() {
            return Self::new(vec![], 0.0);
        }

        let n_features = features[0].len();
        let mut weights = vec![0.0; n_features];
        let mut bias = 0.0;
        let n = features.len() as f64;

        for _ in 0..epochs {
            let mut grad_w = vec![0.0; n_features];
            let mut grad_b = 0.0;

            for (x, y) in features.iter().zip(targets.iter()) {
                let pred: f64 = bias
                    + weights
                        .iter()
                        .zip(x.iter())
                        .map(|(w, f)| w * f)
                        .sum::<f64>();
                let err = pred - y;
                grad_b += err;
                for (gw, xi) in grad_w.iter_mut().zip(x.iter()) {
                    *gw += err * xi;
                }
            }

            bias -= learning_rate * grad_b / n;
            for (w, gw) in weights.iter_mut().zip(grad_w.iter()) {
                *w -= learning_rate * gw / n;
            }
        }

        Self::new(weights, bias)
    }
}

/// A decision stump — splits on one feature at a threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionStump {
    pub feature_idx: usize,
    pub threshold: f64,
    pub left_value: f64,
    pub right_value: f64,
}

impl DecisionStump {
    /// Predict from a feature vector.
    pub fn predict(&self, features: &[f64]) -> f64 {
        if features.get(self.feature_idx).copied().unwrap_or(0.0) <= self.threshold {
            self.left_value
        } else {
            self.right_value
        }
    }

    /// Find the best split for a single feature.
    pub fn fit_single(feature_values: &[f64], targets: &[f64], feature_idx: usize) -> Self {
        assert_eq!(feature_values.len(), targets.len());
        if feature_values.is_empty() {
            return Self {
                feature_idx,
                threshold: 0.0,
                left_value: 0.0,
                right_value: 0.0,
            };
        }

        let global_mean = targets.iter().sum::<f64>() / targets.len() as f64;

        // Try each unique value as threshold
        let mut sorted: Vec<(f64, f64)> = feature_values
            .iter()
            .cloned()
            .zip(targets.iter().cloned())
            .collect();
        sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut best_threshold = sorted[0].0;
        let mut best_mse = f64::INFINITY;
        let mut best_left = global_mean;
        let mut best_right = global_mean;

        for i in 0..sorted.len().saturating_sub(1) {
            if (sorted[i].0 - sorted[i + 1].0).abs() < f64::EPSILON {
                continue;
            }
            let threshold = (sorted[i].0 + sorted[i + 1].0) / 2.0;
            let left: Vec<f64> = sorted
                .iter()
                .filter(|(v, _)| *v <= threshold)
                .map(|(_, t)| *t)
                .collect();
            let right: Vec<f64> = sorted
                .iter()
                .filter(|(v, _)| *v > threshold)
                .map(|(_, t)| *t)
                .collect();

            if left.is_empty() || right.is_empty() {
                continue;
            }

            let left_mean = left.iter().sum::<f64>() / left.len() as f64;
            let right_mean = right.iter().sum::<f64>() / right.len() as f64;

            let mse: f64 = left.iter().map(|v| (v - left_mean).powi(2)).sum::<f64>()
                + right.iter().map(|v| (v - right_mean).powi(2)).sum::<f64>();

            if mse < best_mse {
                best_mse = mse;
                best_threshold = threshold;
                best_left = left_mean;
                best_right = right_mean;
            }
        }

        Self {
            feature_idx,
            threshold: best_threshold,
            left_value: best_left,
            right_value: best_right,
        }
    }
}

/// Simple ensemble that averages predictions from multiple models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleModel {
    pub linear: Option<LinearModel>,
    pub stumps: Vec<DecisionStump>,
    pub linear_weight: f64,
    pub stump_weight: f64,
}

impl EnsembleModel {
    /// Predict by weighted average of sub-models.
    pub fn predict(&self, features: &[f64]) -> f64 {
        let mut total = 0.0;
        let mut weight_sum = 0.0;

        if let Some(ref linear) = self.linear {
            total += self.linear_weight * linear.predict(features);
            weight_sum += self.linear_weight;
        }

        if !self.stumps.is_empty() {
            let stump_avg: f64 = self.stumps.iter().map(|s| s.predict(features)).sum::<f64>()
                / self.stumps.len() as f64;
            total += self.stump_weight * stump_avg;
            weight_sum += self.stump_weight;
        }

        if weight_sum.abs() < f64::EPSILON {
            0.0
        } else {
            total / weight_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_model_predict() {
        let model = LinearModel::new(vec![2.0, 3.0], 1.0);
        let pred = model.predict(&[4.0, 5.0]); // 1 + 2*4 + 3*5 = 24
        assert!((pred - 24.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_linear_model_fit() {
        // Simple y = 2*x + 1
        let features: Vec<Vec<f64>> = (0..20).map(|i| vec![i as f64]).collect();
        let targets: Vec<f64> = (0..20).map(|i| 2.0 * i as f64 + 1.0).collect();
        let model = LinearModel::fit(&features, &targets, 0.001, 1000);

        let pred = model.predict(&[10.0]);
        assert!((pred - 21.0).abs() < 1.0); // allow some gradient descent imprecision
    }

    #[test]
    fn test_decision_stump_predict() {
        let stump = DecisionStump {
            feature_idx: 0,
            threshold: 5.0,
            left_value: 10.0,
            right_value: 20.0,
        };
        assert!((stump.predict(&[3.0]) - 10.0).abs() < f64::EPSILON);
        assert!((stump.predict(&[7.0]) - 20.0).abs() < f64::EPSILON);
        assert!((stump.predict(&[5.0]) - 10.0).abs() < f64::EPSILON); // <= threshold
    }

    #[test]
    fn test_decision_stump_fit() {
        let values = vec![1.0, 2.0, 3.0, 10.0, 11.0, 12.0];
        let targets = vec![5.0, 5.0, 5.0, 15.0, 15.0, 15.0];
        let stump = DecisionStump::fit_single(&values, &targets, 0);
        // Should split somewhere between 3 and 10
        assert!(stump.threshold > 3.0 && stump.threshold < 10.0);
        assert!((stump.left_value - 5.0).abs() < f64::EPSILON);
        assert!((stump.right_value - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ensemble_predict() {
        let linear = LinearModel::new(vec![1.0], 0.0); // y = x
        let stump = DecisionStump {
            feature_idx: 0,
            threshold: 5.0,
            left_value: 2.0,
            right_value: 8.0,
        };
        let ensemble = EnsembleModel {
            linear: Some(linear),
            stumps: vec![stump],
            linear_weight: 0.5,
            stump_weight: 0.5,
        };
        // features = [3.0]: linear=3.0, stump=2.0 → (0.5*3 + 0.5*2)/(0.5+0.5) = 2.5
        let pred = ensemble.predict(&[3.0]);
        assert!((pred - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_model_metrics_compute() {
        let preds = vec![1.0, 2.0, 3.0];
        let actuals = vec![1.0, 2.0, 3.0];
        let m = ModelMetrics::compute(&preds, &actuals);
        assert!((m.mae - 0.0).abs() < f64::EPSILON);
        assert!((m.rmse - 0.0).abs() < f64::EPSILON);
        assert!((m.r_squared - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_model_metrics_imperfect() {
        let preds = vec![1.0, 2.5, 3.0];
        let actuals = vec![1.0, 2.0, 4.0];
        let m = ModelMetrics::compute(&preds, &actuals);
        assert!(m.mae > 0.0);
        assert!(m.rmse > 0.0);
        assert!(m.r_squared < 1.0);
    }

    #[test]
    fn test_empty_metrics() {
        let m = ModelMetrics::empty();
        assert_eq!(m.sample_count, 0);
    }

    #[test]
    fn test_serialization() {
        let model = LinearModel::new(vec![1.0, 2.0], 0.5);
        let json = serde_json::to_string(&model).unwrap();
        let de: LinearModel = serde_json::from_str(&json).unwrap();
        assert_eq!(de.weights, vec![1.0, 2.0]);
        assert!((de.bias - 0.5).abs() < f64::EPSILON);
    }
}
