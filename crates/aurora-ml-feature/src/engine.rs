/// ml feature: extract, transform, select, store, log
/// Phase 1467

#[derive(Debug, Clone)]
pub struct MlFeature {
    pub extract_ok: bool,
    pub transform_ok: bool,
    pub select_ok: bool,
    pub store_ok: bool,
    pub log_ok: bool,
}

impl Default for MlFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl MlFeature {
    pub fn new() -> Self {
        Self {
            extract_ok: true,
            transform_ok: true,
            select_ok: true,
            store_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.extract_ok && self.transform_ok && self.select_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.store_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.extract_ok || !self.transform_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.extract_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MlFeature::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlFeature::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlFeature::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlFeature::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlFeature::new();
        c.extract_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlFeature::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
