/// sas calib: measure, offset, center, validate, log
/// Phase 1387

#[derive(Debug, Clone)]
pub struct SasCalib {
    pub measure_ok: bool,
    pub offset_ok: bool,
    pub center_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for SasCalib {
    fn default() -> Self {
        Self::new()
    }
}

impl SasCalib {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            offset_ok: true,
            center_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.offset_ok && self.center_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.offset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok {
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
        let c = SasCalib::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SasCalib::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SasCalib::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SasCalib::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SasCalib::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SasCalib::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
