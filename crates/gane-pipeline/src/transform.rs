//! Data transformation — field mapping, filtering, enrichment, and validation.

use std::collections::HashMap;

/// Transformation operation type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformOp {
    /// Rename a field.
    Rename { from: String, to: String },
    /// Remove a field.
    Drop(String),
    /// Add a constant field.
    AddConstant { key: String, value: String },
    /// Convert field value to uppercase.
    Uppercase(String),
    /// Convert field value to lowercase.
    Lowercase(String),
    /// Trim whitespace from field value.
    Trim(String),
    /// Filter: keep record only if field matches value.
    FilterEquals { field: String, value: String },
    /// Filter: keep record only if field exists.
    FilterExists(String),
    /// Compute a derived field by concatenating two fields.
    Concat {
        field_a: String,
        field_b: String,
        output: String,
        separator: String,
    },
}

/// A transformation pipeline — ordered list of operations.
pub struct TransformPipeline {
    name: String,
    operations: Vec<TransformOp>,
    records_processed: u64,
    records_dropped: u64,
}

impl TransformPipeline {
    /// Create a new transformation pipeline.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            operations: Vec::new(),
            records_processed: 0,
            records_dropped: 0,
        }
    }

    /// Add an operation to the pipeline.
    pub fn add_op(&mut self, op: TransformOp) {
        self.operations.push(op);
    }

    /// Apply the pipeline to a single record.
    /// Returns None if the record is filtered out.
    pub fn apply(
        &mut self,
        mut fields: HashMap<String, String>,
    ) -> Option<HashMap<String, String>> {
        self.records_processed += 1;

        for op in &self.operations {
            match op {
                TransformOp::Rename { from, to } => {
                    if let Some(val) = fields.remove(from) {
                        fields.insert(to.clone(), val);
                    }
                }
                TransformOp::Drop(key) => {
                    fields.remove(key);
                }
                TransformOp::AddConstant { key, value } => {
                    fields.insert(key.clone(), value.clone());
                }
                TransformOp::Uppercase(key) => {
                    if let Some(val) = fields.get_mut(key) {
                        *val = val.to_uppercase();
                    }
                }
                TransformOp::Lowercase(key) => {
                    if let Some(val) = fields.get_mut(key) {
                        *val = val.to_lowercase();
                    }
                }
                TransformOp::Trim(key) => {
                    if let Some(val) = fields.get_mut(key) {
                        *val = val.trim().to_string();
                    }
                }
                TransformOp::FilterEquals { field, value } => {
                    if fields.get(field).map(|v| v.as_str()) != Some(value.as_str()) {
                        self.records_dropped += 1;
                        return None;
                    }
                }
                TransformOp::FilterExists(field) => {
                    if !fields.contains_key(field) {
                        self.records_dropped += 1;
                        return None;
                    }
                }
                TransformOp::Concat {
                    field_a,
                    field_b,
                    output,
                    separator,
                } => {
                    let a = fields.get(field_a).cloned().unwrap_or_default();
                    let b = fields.get(field_b).cloned().unwrap_or_default();
                    fields.insert(output.clone(), format!("{a}{separator}{b}"));
                }
            }
        }

        Some(fields)
    }

    /// Apply the pipeline to a batch of records.
    pub fn apply_batch(
        &mut self,
        batch: Vec<HashMap<String, String>>,
    ) -> Vec<HashMap<String, String>> {
        batch.into_iter().filter_map(|r| self.apply(r)).collect()
    }

    /// Get pipeline name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get number of operations.
    pub fn op_count(&self) -> usize {
        self.operations.len()
    }

    /// Get total records processed.
    pub fn records_processed(&self) -> u64 {
        self.records_processed
    }

    /// Get total records dropped by filters.
    pub fn records_dropped(&self) -> u64 {
        self.records_dropped
    }

    /// Get pass-through rate (records that survived filters).
    pub fn pass_rate(&self) -> f64 {
        if self.records_processed == 0 {
            return 1.0;
        }
        (self.records_processed - self.records_dropped) as f64 / self.records_processed as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fields(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn test_rename_field() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::Rename {
            from: "lat".to_string(),
            to: "latitude".to_string(),
        });
        let result = pipe.apply(make_fields(&[("lat", "40.7")])).unwrap();
        assert_eq!(result.get("latitude").unwrap(), "40.7");
        assert!(!result.contains_key("lat"));
    }

    #[test]
    fn test_drop_field() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::Drop("secret".to_string()));
        let result = pipe
            .apply(make_fields(&[("name", "a"), ("secret", "x")]))
            .unwrap();
        assert!(!result.contains_key("secret"));
        assert!(result.contains_key("name"));
    }

    #[test]
    fn test_add_constant() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::AddConstant {
            key: "version".to_string(),
            value: "2.0".to_string(),
        });
        let result = pipe.apply(make_fields(&[])).unwrap();
        assert_eq!(result.get("version").unwrap(), "2.0");
    }

    #[test]
    fn test_uppercase() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::Uppercase("city".to_string()));
        let result = pipe.apply(make_fields(&[("city", "tokyo")])).unwrap();
        assert_eq!(result.get("city").unwrap(), "TOKYO");
    }

    #[test]
    fn test_lowercase() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::Lowercase("code".to_string()));
        let result = pipe.apply(make_fields(&[("code", "ABC")])).unwrap();
        assert_eq!(result.get("code").unwrap(), "abc");
    }

    #[test]
    fn test_trim() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::Trim("name".to_string()));
        let result = pipe.apply(make_fields(&[("name", "  hello  ")])).unwrap();
        assert_eq!(result.get("name").unwrap(), "hello");
    }

    #[test]
    fn test_filter_equals_keeps() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::FilterEquals {
            field: "type".to_string(),
            value: "gnss".to_string(),
        });
        let result = pipe.apply(make_fields(&[("type", "gnss")]));
        assert!(result.is_some());
    }

    #[test]
    fn test_filter_equals_drops() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::FilterEquals {
            field: "type".to_string(),
            value: "gnss".to_string(),
        });
        let result = pipe.apply(make_fields(&[("type", "imu")]));
        assert!(result.is_none());
        assert_eq!(pipe.records_dropped(), 1);
    }

    #[test]
    fn test_filter_exists_drops_missing() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::FilterExists("required_field".to_string()));
        let result = pipe.apply(make_fields(&[("other", "val")]));
        assert!(result.is_none());
    }

    #[test]
    fn test_concat_fields() {
        let mut pipe = TransformPipeline::new("test");
        pipe.add_op(TransformOp::Concat {
            field_a: "first".to_string(),
            field_b: "last".to_string(),
            output: "full_name".to_string(),
            separator: " ".to_string(),
        });
        let result = pipe
            .apply(make_fields(&[("first", "John"), ("last", "Doe")]))
            .unwrap();
        assert_eq!(result.get("full_name").unwrap(), "John Doe");
    }

    #[test]
    fn test_chained_operations() {
        let mut pipe = TransformPipeline::new("chain");
        pipe.add_op(TransformOp::Rename {
            from: "lat".to_string(),
            to: "latitude".to_string(),
        });
        pipe.add_op(TransformOp::Uppercase("city".to_string()));
        pipe.add_op(TransformOp::Drop("temp".to_string()));
        let result = pipe
            .apply(make_fields(&[
                ("lat", "40.7"),
                ("city", "nyc"),
                ("temp", "x"),
            ]))
            .unwrap();
        assert_eq!(result.get("latitude").unwrap(), "40.7");
        assert_eq!(result.get("city").unwrap(), "NYC");
        assert!(!result.contains_key("temp"));
        assert!(!result.contains_key("lat"));
    }

    #[test]
    fn test_batch_processing() {
        let mut pipe = TransformPipeline::new("batch");
        pipe.add_op(TransformOp::FilterEquals {
            field: "status".to_string(),
            value: "active".to_string(),
        });
        let batch = vec![
            make_fields(&[("status", "active"), ("id", "1")]),
            make_fields(&[("status", "inactive"), ("id", "2")]),
            make_fields(&[("status", "active"), ("id", "3")]),
        ];
        let results = pipe.apply_batch(batch);
        assert_eq!(results.len(), 2);
        assert_eq!(pipe.records_processed(), 3);
        assert_eq!(pipe.records_dropped(), 1);
    }

    #[test]
    fn test_pass_rate() {
        let mut pipe = TransformPipeline::new("rate");
        pipe.add_op(TransformOp::FilterExists("x".to_string()));
        pipe.apply(make_fields(&[("x", "1")])); // passes
        pipe.apply(make_fields(&[("y", "2")])); // dropped
        pipe.apply(make_fields(&[("x", "3")])); // passes
        pipe.apply(make_fields(&[("z", "4")])); // dropped
        assert_eq!(pipe.records_processed(), 4);
        assert_eq!(pipe.records_dropped(), 2);
        assert!((pipe.pass_rate() - 0.5).abs() < f64::EPSILON);
    }
}
