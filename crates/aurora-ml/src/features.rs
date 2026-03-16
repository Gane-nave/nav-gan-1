//! Feature engineering — extracts, transforms, and normalises raw navigation
//! data into feature vectors suitable for ML models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single feature value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureValue {
    Numeric(f64),
    Categorical(String),
    Boolean(bool),
    Vector(Vec<f64>),
}

/// A named feature extracted from raw data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub value: FeatureValue,
}

/// A complete feature vector for a single sample.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureVector {
    pub features: Vec<Feature>,
    pub label: Option<f64>,
}

impl FeatureVector {
    /// Create a new empty feature vector.
    pub fn new() -> Self {
        Self {
            features: Vec::new(),
            label: None,
        }
    }

    /// Add a numeric feature.
    pub fn add_numeric(&mut self, name: &str, value: f64) {
        self.features.push(Feature {
            name: name.to_string(),
            value: FeatureValue::Numeric(value),
        });
    }

    /// Add a categorical feature.
    pub fn add_categorical(&mut self, name: &str, value: &str) {
        self.features.push(Feature {
            name: name.to_string(),
            value: FeatureValue::Categorical(value.to_string()),
        });
    }

    /// Add a boolean feature.
    pub fn add_boolean(&mut self, name: &str, value: bool) {
        self.features.push(Feature {
            name: name.to_string(),
            value: FeatureValue::Boolean(value),
        });
    }

    /// Set the label (target variable).
    pub fn with_label(mut self, label: f64) -> Self {
        self.label = Some(label);
        self
    }

    /// Convert to a flat numeric vector, encoding categoricals as hashes
    /// and flattening vector features into individual elements.
    pub fn to_numeric_vec(&self) -> Vec<f64> {
        self.features
            .iter()
            .flat_map(|f| match &f.value {
                FeatureValue::Numeric(v) => vec![*v],
                FeatureValue::Boolean(b) => {
                    if *b {
                        vec![1.0]
                    } else {
                        vec![0.0]
                    }
                }
                FeatureValue::Categorical(s) => {
                    // Simple hash-based encoding
                    let hash: u32 = s
                        .bytes()
                        .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                    vec![(hash % 1000) as f64 / 1000.0]
                }
                FeatureValue::Vector(v) => v.clone(),
            })
            .collect()
    }

    /// Get a feature by name.
    pub fn get(&self, name: &str) -> Option<&FeatureValue> {
        self.features
            .iter()
            .find(|f| f.name == name)
            .map(|f| &f.value)
    }

    /// Number of features.
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Whether the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }
}

impl Default for FeatureVector {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalisation strategy for numeric features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormStrategy {
    /// Min-max scaling to [0, 1].
    MinMax,
    /// Z-score normalisation (mean=0, std=1).
    ZScore,
    /// No normalisation.
    None,
}

/// Feature normaliser that learns statistics from training data and applies
/// the same transformation to new data.
pub struct FeatureNormaliser {
    strategy: NormStrategy,
    stats: HashMap<String, FeatureStats>,
}

#[derive(Debug, Clone)]
struct FeatureStats {
    min: f64,
    max: f64,
    mean: f64,
    std_dev: f64,
}

impl FeatureNormaliser {
    /// Create a new normaliser.
    pub fn new(strategy: NormStrategy) -> Self {
        Self {
            strategy,
            stats: HashMap::new(),
        }
    }

    /// Fit the normaliser on training data.
    pub fn fit(&mut self, samples: &[FeatureVector]) {
        if samples.is_empty() {
            return;
        }

        // Collect values per feature name
        let mut values_by_name: HashMap<String, Vec<f64>> = HashMap::new();
        for sample in samples {
            for f in &sample.features {
                if let FeatureValue::Numeric(v) = &f.value {
                    values_by_name.entry(f.name.clone()).or_default().push(*v);
                }
            }
        }

        for (name, values) in &values_by_name {
            let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance =
                values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
            let std_dev = variance.sqrt();

            self.stats.insert(
                name.clone(),
                FeatureStats {
                    min,
                    max,
                    mean,
                    std_dev,
                },
            );
        }
    }

    /// Transform a feature vector using learned statistics.
    pub fn transform(&self, fv: &FeatureVector) -> FeatureVector {
        let mut result = FeatureVector {
            features: Vec::new(),
            label: fv.label,
        };

        for f in &fv.features {
            let transformed = match (&f.value, self.stats.get(&f.name)) {
                (FeatureValue::Numeric(v), Some(stats)) => match self.strategy {
                    NormStrategy::MinMax => {
                        let range = stats.max - stats.min;
                        let normed = if range.abs() < f64::EPSILON {
                            0.0
                        } else {
                            (v - stats.min) / range
                        };
                        FeatureValue::Numeric(normed)
                    }
                    NormStrategy::ZScore => {
                        let normed = if stats.std_dev.abs() < f64::EPSILON {
                            0.0
                        } else {
                            (v - stats.mean) / stats.std_dev
                        };
                        FeatureValue::Numeric(normed)
                    }
                    NormStrategy::None => FeatureValue::Numeric(*v),
                },
                _ => f.value.clone(),
            };

            result.features.push(Feature {
                name: f.name.clone(),
                value: transformed,
            });
        }

        result
    }

    /// Get the strategy.
    pub fn strategy(&self) -> NormStrategy {
        self.strategy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_vector_numeric() {
        let mut fv = FeatureVector::new();
        fv.add_numeric("speed", 60.0);
        fv.add_numeric("distance", 100.0);
        assert_eq!(fv.len(), 2);
        assert!(
            matches!(fv.get("speed"), Some(FeatureValue::Numeric(v)) if (*v - 60.0).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn test_feature_vector_mixed() {
        let mut fv = FeatureVector::new();
        fv.add_numeric("speed", 60.0);
        fv.add_categorical("road_type", "highway");
        fv.add_boolean("is_rush_hour", true);
        let vec = fv.to_numeric_vec();
        assert_eq!(vec.len(), 3);
        assert!((vec[0] - 60.0).abs() < f64::EPSILON);
        assert!((vec[2] - 1.0).abs() < f64::EPSILON); // true -> 1.0
    }

    #[test]
    fn test_min_max_normalisation() {
        let samples: Vec<FeatureVector> = (0..5)
            .map(|i| {
                let mut fv = FeatureVector::new();
                fv.add_numeric("x", i as f64 * 10.0); // 0, 10, 20, 30, 40
                fv
            })
            .collect();

        let mut norm = FeatureNormaliser::new(NormStrategy::MinMax);
        norm.fit(&samples);

        let test = &samples[2]; // x=20
        let normed = norm.transform(test);
        if let Some(FeatureValue::Numeric(v)) = normed.get("x") {
            assert!((*v - 0.5).abs() < f64::EPSILON); // (20-0)/(40-0) = 0.5
        } else {
            panic!("expected numeric");
        }
    }

    #[test]
    fn test_zscore_normalisation() {
        // Values: 10, 20, 30 → mean=20, std=~8.165
        let samples: Vec<FeatureVector> = [10.0, 20.0, 30.0]
            .iter()
            .map(|v| {
                let mut fv = FeatureVector::new();
                fv.add_numeric("x", *v);
                fv
            })
            .collect();

        let mut norm = FeatureNormaliser::new(NormStrategy::ZScore);
        norm.fit(&samples);

        let test = &samples[1]; // x=20 (mean)
        let normed = norm.transform(test);
        if let Some(FeatureValue::Numeric(v)) = normed.get("x") {
            assert!(v.abs() < 1e-10); // mean value → z-score 0
        } else {
            panic!("expected numeric");
        }
    }

    #[test]
    fn test_constant_feature_normalisation() {
        let samples: Vec<FeatureVector> = (0..3)
            .map(|_| {
                let mut fv = FeatureVector::new();
                fv.add_numeric("const", 5.0);
                fv
            })
            .collect();

        let mut norm = FeatureNormaliser::new(NormStrategy::MinMax);
        norm.fit(&samples);
        let normed = norm.transform(&samples[0]);
        if let Some(FeatureValue::Numeric(v)) = normed.get("const") {
            assert!((*v - 0.0).abs() < f64::EPSILON); // zero range → 0
        } else {
            panic!("expected numeric");
        }
    }

    #[test]
    fn test_label_preserved() {
        let fv = FeatureVector::new().with_label(42.0);
        assert!((fv.label.unwrap() - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_empty_vector() {
        let fv = FeatureVector::new();
        assert!(fv.is_empty());
        assert_eq!(fv.to_numeric_vec().len(), 0);
    }

    #[test]
    fn test_serialization() {
        let mut fv = FeatureVector::new();
        fv.add_numeric("speed", 55.0);
        fv.add_categorical("type", "urban");
        let json = serde_json::to_string(&fv).unwrap();
        let de: FeatureVector = serde_json::from_str(&json).unwrap();
        assert_eq!(de.len(), 2);
    }
}
