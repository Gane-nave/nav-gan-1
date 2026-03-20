/// clutch ctrl: engage, slip, release, wear, report
/// Phase 1217

#[derive(Debug, Clone)]
pub struct ClutchCtrl {
    pub engage_ok: bool,
    pub slip_ok: bool,
    pub release_ok: bool,
    pub wear_ok: bool,
    pub report_ok: bool,
}

impl Default for ClutchCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl ClutchCtrl {
    pub fn new() -> Self {
        Self {
            engage_ok: true,
            slip_ok: true,
            release_ok: true,
            wear_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.engage_ok && self.slip_ok && self.release_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.wear_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.engage_ok || !self.slip_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.engage_ok {
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
        let c = ClutchCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ClutchCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ClutchCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ClutchCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ClutchCtrl::new();
        c.engage_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ClutchCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
