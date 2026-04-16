/// regen brake: capture, convert, store, limit, report
/// Phase 1143

#[derive(Debug, Clone)]
pub struct RegenBrake {
    pub capture_ok: bool,
    pub convert_ok: bool,
    pub store_ok: bool,
    pub limit_ok: bool,
    pub report_ok: bool,
}

impl Default for RegenBrake {
    fn default() -> Self {
        Self::new()
    }
}

impl RegenBrake {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            convert_ok: true,
            store_ok: true,
            limit_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.convert_ok && self.store_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.limit_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.convert_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok {
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
        let c = RegenBrake::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RegenBrake::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RegenBrake::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RegenBrake::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RegenBrake::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RegenBrake::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
