/// data enrich2: lookup, join, derive, validate, log
/// Phase 2207

#[derive(Debug, Clone)]
pub struct DataEnrich2 {
    pub lookup_ok: bool,
    pub join_ok: bool,
    pub derive_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for DataEnrich2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataEnrich2 {
    pub fn new() -> Self {
        Self {
            lookup_ok: true,
            join_ok: true,
            derive_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.lookup_ok && self.join_ok && self.derive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.lookup_ok || !self.join_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lookup_ok {
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
        let c = DataEnrich2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataEnrich2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataEnrich2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataEnrich2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataEnrich2::new();
        c.lookup_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataEnrich2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
