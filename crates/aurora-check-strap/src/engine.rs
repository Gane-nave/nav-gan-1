/// Check strap: door hold-open, detent positions, strap condition
/// Phase 412

#[derive(Debug, Clone)]
pub struct CheckStrap {
    pub detent_count: u8,
    pub hold_force_n: f64,
    pub min_force_n: f64,
    pub strap_ok: bool,
    pub pin_ok: bool,
}

impl Default for CheckStrap {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckStrap {
    pub fn new() -> Self {
        Self {
            detent_count: 3,
            hold_force_n: 25.0,
            min_force_n: 15.0,
            strap_ok: true,
            pin_ok: true,
        }
    }

    pub fn hold_ok(&self) -> bool {
        self.hold_force_n >= self.min_force_n
    }

    pub fn all_ok(&self) -> bool {
        self.hold_ok() && self.strap_ok && self.pin_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.strap_ok || !self.pin_ok
    }

    pub fn force_margin_pct(&self) -> f64 {
        if self.min_force_n <= 0.0 {
            return 0.0;
        }
        ((self.hold_force_n / self.min_force_n - 1.0) * 100.0).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.strap_ok {
            return 0.0;
        }
        if !self.hold_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hold() {
        let c = CheckStrap::new();
        assert!(c.hold_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CheckStrap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CheckStrap::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_margin() {
        let c = CheckStrap::new();
        assert!(c.force_margin_pct() > 60.0);
    }

    #[test]
    fn test_broken() {
        let mut c = CheckStrap::new();
        c.strap_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CheckStrap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
