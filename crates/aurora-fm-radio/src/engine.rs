/// fm radio: tune, demod, rds, scan, log
/// Phase 1348

#[derive(Debug, Clone)]
pub struct FmRadio {
    pub tune_ok: bool,
    pub demod_ok: bool,
    pub rds_ok: bool,
    pub scan_ok: bool,
    pub log_ok: bool,
}

impl Default for FmRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl FmRadio {
    pub fn new() -> Self {
        Self {
            tune_ok: true,
            demod_ok: true,
            rds_ok: true,
            scan_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.tune_ok && self.demod_ok && self.rds_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.scan_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.tune_ok || !self.demod_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tune_ok {
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
        let c = FmRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FmRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FmRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FmRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FmRadio::new();
        c.tune_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FmRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
