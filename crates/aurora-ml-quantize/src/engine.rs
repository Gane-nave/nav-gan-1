/// ml quantize: convert, calibrate, evaluate, export, log
/// Phase 1967

#[derive(Debug, Clone)]
pub struct MlQuantize {
    pub convert_ok: bool,
    pub calibrate_ok: bool,
    pub evaluate_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for MlQuantize {
    fn default() -> Self {
        Self::new()
    }
}

impl MlQuantize {
    pub fn new() -> Self {
        Self {
            convert_ok: true,
            calibrate_ok: true,
            evaluate_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.convert_ok && self.calibrate_ok && self.evaluate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.convert_ok || !self.calibrate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.convert_ok {
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
        let c = MlQuantize::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlQuantize::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlQuantize::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlQuantize::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlQuantize::new();
        c.convert_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlQuantize::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
