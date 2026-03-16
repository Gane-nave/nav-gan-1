//! Training pipeline — orchestrates data splitting, model training,
//! validation, and model selection.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::FeatureVector;
use crate::models::{LinearModel, ModelMetrics, ModelStatus, ModelType};

/// Configuration for a training run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub model_type: ModelType,
    pub learning_rate: f64,
    pub epochs: usize,
    /// Fraction of data used for validation (0.0 - 1.0).
    pub validation_split: f64,
    /// Minimum R-squared to consider the model acceptable.
    pub min_r_squared: f64,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            model_type: ModelType::LinearRegression,
            learning_rate: 0.01,
            epochs: 500,
            validation_split: 0.2,
            min_r_squared: 0.5,
        }
    }
}

/// Result of a training run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub run_id: Uuid,
    pub model_type: ModelType,
    pub status: ModelStatus,
    pub train_metrics: ModelMetrics,
    pub val_metrics: ModelMetrics,
    pub config: TrainingConfig,
    pub timestamp: String,
}

/// Split data into training and validation sets.
pub fn train_val_split(
    data: &[FeatureVector],
    val_fraction: f64,
) -> (Vec<FeatureVector>, Vec<FeatureVector>) {
    let val_fraction = val_fraction.clamp(0.0, 1.0);
    let split_idx = ((1.0 - val_fraction) * data.len() as f64) as usize;
    let train = data[..split_idx].to_vec();
    let val = data[split_idx..].to_vec();
    (train, val)
}

/// Run a complete training pipeline.
pub fn run_training(data: &[FeatureVector], config: &TrainingConfig) -> TrainingResult {
    let run_id = Uuid::new_v4();

    if data.is_empty() {
        return TrainingResult {
            run_id,
            model_type: config.model_type,
            status: ModelStatus::Failed,
            train_metrics: ModelMetrics::empty(),
            val_metrics: ModelMetrics::empty(),
            config: config.clone(),
            timestamp: Utc::now().to_rfc3339(),
        };
    }

    let (train_data, val_data) = train_val_split(data, config.validation_split);

    // Extract features and labels
    let train_features: Vec<Vec<f64>> = train_data.iter().map(|fv| fv.to_numeric_vec()).collect();
    let train_labels: Vec<f64> = train_data
        .iter()
        .map(|fv| fv.label.unwrap_or(0.0))
        .collect();

    let val_features: Vec<Vec<f64>> = val_data.iter().map(|fv| fv.to_numeric_vec()).collect();
    let val_labels: Vec<f64> = val_data.iter().map(|fv| fv.label.unwrap_or(0.0)).collect();

    // Train the model
    let model = LinearModel::fit(
        &train_features,
        &train_labels,
        config.learning_rate,
        config.epochs,
    );

    // Evaluate on train set
    let train_preds: Vec<f64> = train_features.iter().map(|f| model.predict(f)).collect();
    let train_metrics = ModelMetrics::compute(&train_preds, &train_labels);

    // Evaluate on validation set
    let val_preds: Vec<f64> = val_features.iter().map(|f| model.predict(f)).collect();
    let val_metrics = if val_labels.is_empty() {
        ModelMetrics::empty()
    } else {
        ModelMetrics::compute(&val_preds, &val_labels)
    };

    let status = if val_metrics.r_squared >= config.min_r_squared || val_labels.is_empty() {
        ModelStatus::Ready
    } else {
        ModelStatus::Failed
    };

    TrainingResult {
        run_id,
        model_type: config.model_type,
        status,
        train_metrics,
        val_metrics,
        config: config.clone(),
        timestamp: Utc::now().to_rfc3339(),
    }
}

/// Cross-validation: run k-fold training and return average metrics.
pub fn cross_validate(
    data: &[FeatureVector],
    config: &TrainingConfig,
    k: usize,
) -> Vec<ModelMetrics> {
    if data.is_empty() || k == 0 {
        return Vec::new();
    }

    let k = k.min(data.len());
    let fold_size = data.len() / k;
    let mut fold_metrics = Vec::new();

    for fold in 0..k {
        let start = fold * fold_size;
        let end = if fold == k - 1 {
            data.len()
        } else {
            start + fold_size
        };

        let val_data = &data[start..end];
        let train_data: Vec<FeatureVector> = data[..start]
            .iter()
            .chain(data[end..].iter())
            .cloned()
            .collect();

        let train_features: Vec<Vec<f64>> =
            train_data.iter().map(|fv| fv.to_numeric_vec()).collect();
        let train_labels: Vec<f64> = train_data
            .iter()
            .map(|fv| fv.label.unwrap_or(0.0))
            .collect();

        if train_features.is_empty() {
            continue;
        }

        let model = LinearModel::fit(
            &train_features,
            &train_labels,
            config.learning_rate,
            config.epochs,
        );

        let val_preds: Vec<f64> = val_data
            .iter()
            .map(|fv| model.predict(&fv.to_numeric_vec()))
            .collect();
        let val_labels: Vec<f64> = val_data.iter().map(|fv| fv.label.unwrap_or(0.0)).collect();
        fold_metrics.push(ModelMetrics::compute(&val_preds, &val_labels));
    }

    fold_metrics
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linear_dataset(n: usize) -> Vec<FeatureVector> {
        (0..n)
            .map(|i| {
                let x = i as f64;
                let mut fv = FeatureVector::new();
                fv.add_numeric("x", x);
                fv.with_label(2.0 * x + 1.0)
            })
            .collect()
    }

    #[test]
    fn test_train_val_split() {
        let data = linear_dataset(100);
        let (train, val) = train_val_split(&data, 0.2);
        assert_eq!(train.len(), 80);
        assert_eq!(val.len(), 20);
    }

    #[test]
    fn test_run_training_success() {
        let data = linear_dataset(50);
        let config = TrainingConfig {
            learning_rate: 0.0001,
            epochs: 2000,
            validation_split: 0.2,
            min_r_squared: 0.5,
            ..Default::default()
        };
        let result = run_training(&data, &config);
        assert_eq!(result.status, ModelStatus::Ready);
        assert!(result.train_metrics.r_squared > 0.5);
    }

    #[test]
    fn test_run_training_empty_data() {
        let config = TrainingConfig::default();
        let result = run_training(&[], &config);
        assert_eq!(result.status, ModelStatus::Failed);
    }

    #[test]
    fn test_cross_validate() {
        let data = linear_dataset(30);
        let config = TrainingConfig {
            learning_rate: 0.0001,
            epochs: 1000,
            ..Default::default()
        };
        let metrics = cross_validate(&data, &config, 3);
        assert_eq!(metrics.len(), 3);
        for m in &metrics {
            assert!(m.sample_count > 0);
        }
    }

    #[test]
    fn test_cross_validate_empty() {
        let config = TrainingConfig::default();
        let metrics = cross_validate(&[], &config, 5);
        assert!(metrics.is_empty());
    }

    #[test]
    fn test_default_config() {
        let config = TrainingConfig::default();
        assert_eq!(config.model_type, ModelType::LinearRegression);
        assert!((config.validation_split - 0.2).abs() < f64::EPSILON);
    }

    #[test]
    fn test_serialization() {
        let config = TrainingConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let de: TrainingConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(de.epochs, config.epochs);
    }
}
