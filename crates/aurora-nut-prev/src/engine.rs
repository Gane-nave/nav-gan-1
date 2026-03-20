/// Nut prevailing: nylon lock nut, all-metal prevailing, castle nut
/// Phase 405

#[derive(Debug, Clone)]
pub struct NutPrev {
    pub prevailing_torque_nm: f64,
    pub min_prevailing_nm: f64,
    pub nylon_insert: bool,
    pub reusable: bool,
    pub use_count: u32,
}

impl Default for NutPrev {
    fn default() -> Self {
        Self::new()
    }
}

impl NutPrev {
    pub fn new() -> Self {
        Self {
            prevailing_torque_nm: 3.0,
            min_prevailing_nm: 1.5,
            nylon_insert: true,
            reusable: true,
            use_count: 1,
        }
    }

    pub fn locking_ok(&self) -> bool {
        self.prevailing_torque_nm >= self.min_prevailing_nm
    }

    pub fn can_reuse(&self) -> bool {
        self.reusable && self.use_count < 5
    }

    pub fn needs_replacement(&self) -> bool {
        !self.locking_ok() || (self.nylon_insert && self.use_count > 3)
    }

    pub fn margin_pct(&self) -> f64 {
        if self.min_prevailing_nm <= 0.0 {
            return 0.0;
        }
        ((self.prevailing_torque_nm / self.min_prevailing_nm - 1.0) * 100.0).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.locking_ok() {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locking() {
        let n = NutPrev::new();
        assert!(n.locking_ok());
    }

    #[test]
    fn test_reuse() {
        let n = NutPrev::new();
        assert!(n.can_reuse());
    }

    #[test]
    fn test_no_replace() {
        let n = NutPrev::new();
        assert!(!n.needs_replacement());
    }

    #[test]
    fn test_margin() {
        let n = NutPrev::new();
        assert!(n.margin_pct() > 90.0);
    }

    #[test]
    fn test_worn() {
        let mut n = NutPrev::new();
        n.prevailing_torque_nm = 0.5;
        assert!(n.needs_replacement());
    }

    #[test]
    fn test_health() {
        let n = NutPrev::new();
        assert!((n.health_score() - 100.0).abs() < 0.1);
    }
}
