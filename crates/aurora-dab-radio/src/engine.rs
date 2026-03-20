/// dab radio: tune, decode, ensemble, scan, log
/// Phase 1349

#[derive(Debug, Clone)]
pub struct DabRadio {
    pub tune_ok: bool,
    pub decode_ok: bool,
    pub ensemble_ok: bool,
    pub scan_ok: bool,
    pub log_ok: bool,
}

impl Default for DabRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl DabRadio {
    pub fn new() -> Self {
        Self {
            tune_ok: true,
            decode_ok: true,
            ensemble_ok: true,
            scan_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.tune_ok && self.decode_ok && self.ensemble_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.scan_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.tune_ok || !self.decode_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tune_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DabRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DabRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DabRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DabRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DabRadio::new();
        c.tune_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DabRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
