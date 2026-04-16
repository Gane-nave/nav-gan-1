/// feature segment: define, match, update, delete, log
/// Phase 1783

#[derive(Debug, Clone)]
pub struct FeatureSegment {
    pub define_ok: bool,
    pub match_ok: bool,
    pub update_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for FeatureSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureSegment {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            match_ok: true,
            update_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.match_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.match_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = FeatureSegment::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FeatureSegment::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FeatureSegment::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FeatureSegment::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FeatureSegment::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FeatureSegment::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
