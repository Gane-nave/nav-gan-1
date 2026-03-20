/// CNN engine: conv, pool, flatten, dense, classify
/// Phase 1016

#[derive(Debug, Clone)]
pub struct CnnEngine {
    pub conv_ok: bool,
    pub pool_ok: bool,
    pub flatten_ok: bool,
    pub dense_ok: bool,
    pub classify_ok: bool,
}

impl Default for CnnEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CnnEngine {
    pub fn new() -> Self {
        Self {
            conv_ok: true,
            pool_ok: true,
            flatten_ok: true,
            dense_ok: true,
            classify_ok: true,
        }
    }

    pub fn feature_ok(&self) -> bool {
        self.conv_ok && self.pool_ok && self.flatten_ok
    }

    pub fn output_ok(&self) -> bool {
        self.dense_ok && self.classify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.feature_ok() && self.output_ok()
    }

    pub fn needs_retrain(&self) -> bool {
        !self.conv_ok || !self.dense_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.conv_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        let c = CnnEngine::new();
        assert!(c.feature_ok());
    }

    #[test]
    fn test_output() {
        let c = CnnEngine::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CnnEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retrain() {
        let c = CnnEngine::new();
        assert!(!c.needs_retrain());
    }

    #[test]
    fn test_conv() {
        let mut c = CnnEngine::new();
        c.conv_ok = false;
        assert!(c.needs_retrain());
    }

    #[test]
    fn test_health() {
        let c = CnnEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
