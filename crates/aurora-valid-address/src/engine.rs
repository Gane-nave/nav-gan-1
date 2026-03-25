/// valid address: parse, normalize, geocode, verify, log
/// Phase 2083

#[derive(Debug, Clone)]
pub struct ValidAddress {
    pub parse_ok: bool,
    pub normalize_ok: bool,
    pub geocode_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidAddress {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidAddress {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            normalize_ok: true,
            geocode_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.normalize_ok && self.geocode_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.normalize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = ValidAddress::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidAddress::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidAddress::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidAddress::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidAddress::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidAddress::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
