/// abs diag: bleed, test, calibrate, validate, log
/// Phase 1389

#[derive(Debug, Clone)]
pub struct AbsDiag {
    pub bleed_ok: bool,
    pub test_ok: bool,
    pub calibrate_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for AbsDiag {
    fn default() -> Self {
        Self::new()
    }
}

impl AbsDiag {
    pub fn new() -> Self {
        Self {
            bleed_ok: true,
            test_ok: true,
            calibrate_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.bleed_ok && self.test_ok && self.calibrate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.bleed_ok || !self.test_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bleed_ok {
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
        let c = AbsDiag::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AbsDiag::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AbsDiag::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AbsDiag::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AbsDiag::new();
        c.bleed_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AbsDiag::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
