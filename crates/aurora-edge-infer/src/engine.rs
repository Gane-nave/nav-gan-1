/// edge inference: load, preprocess, infer, postprocess, cache
/// Phase 1136

#[derive(Debug, Clone)]
pub struct EdgeInfer {
    pub load_ok: bool,
    pub preprocess_ok: bool,
    pub infer_ok: bool,
    pub postprocess_ok: bool,
    pub cache_ok: bool,
}

impl Default for EdgeInfer {
    fn default() -> Self {
        Self::new()
    }
}

impl EdgeInfer {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            preprocess_ok: true,
            infer_ok: true,
            postprocess_ok: true,
            cache_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.preprocess_ok && self.infer_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.postprocess_ok && self.cache_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.preprocess_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = EdgeInfer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EdgeInfer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EdgeInfer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EdgeInfer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EdgeInfer::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EdgeInfer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
