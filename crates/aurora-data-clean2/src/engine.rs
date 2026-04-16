/// data clean2: dedupe, normalize, impute, validate, log
/// Phase 2206

#[derive(Debug, Clone)]
pub struct DataClean2 {
    pub dedupe_ok: bool,
    pub normalize_ok: bool,
    pub impute_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for DataClean2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataClean2 {
    pub fn new() -> Self {
        Self {
            dedupe_ok: true,
            normalize_ok: true,
            impute_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.dedupe_ok && self.normalize_ok && self.impute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.dedupe_ok || !self.normalize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.dedupe_ok {
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
        let c = DataClean2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataClean2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataClean2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataClean2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataClean2::new();
        c.dedupe_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataClean2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
