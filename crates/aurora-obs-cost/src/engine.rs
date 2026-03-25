/// obs cost: track, allocate, optimize, report, log
/// Phase 2159

#[derive(Debug, Clone)]
pub struct ObsCost {
    pub track_ok: bool,
    pub allocate_ok: bool,
    pub optimize_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsCost {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsCost {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            allocate_ok: true,
            optimize_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.track_ok && self.allocate_ok && self.optimize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.track_ok || !self.allocate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.track_ok {
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
        let c = ObsCost::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsCost::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsCost::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsCost::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsCost::new();
        c.track_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsCost::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
