/// mode6 test: request, decode, evaluate, report, log
/// Phase 1378

#[derive(Debug, Clone)]
pub struct Mode6Test {
    pub request_ok: bool,
    pub decode_ok: bool,
    pub evaluate_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for Mode6Test {
    fn default() -> Self {
        Self::new()
    }
}

impl Mode6Test {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            decode_ok: true,
            evaluate_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.decode_ok && self.evaluate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.decode_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = Mode6Test::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Mode6Test::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Mode6Test::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Mode6Test::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Mode6Test::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Mode6Test::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
