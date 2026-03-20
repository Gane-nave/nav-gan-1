/// ml infer: load, preprocess, predict, postprocess, log
/// Phase 1464

#[derive(Debug, Clone)]
pub struct MlInfer {
    pub load_ok: bool,
    pub preprocess_ok: bool,
    pub predict_ok: bool,
    pub postprocess_ok: bool,
    pub log_ok: bool,
}

impl Default for MlInfer {
    fn default() -> Self {
        Self::new()
    }
}

impl MlInfer {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            preprocess_ok: true,
            predict_ok: true,
            postprocess_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.preprocess_ok && self.predict_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.postprocess_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.preprocess_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MlInfer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlInfer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlInfer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlInfer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlInfer::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlInfer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
