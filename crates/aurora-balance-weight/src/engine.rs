/// Balance weight: clip-on, adhesive, position, gram
/// Phase 812

#[derive(Debug, Clone)]
pub struct BalanceWeight {
    pub clip_ok: bool,
    pub adhesive_ok: bool,
    pub position_ok: bool,
    pub gram_ok: bool,
    pub secure_ok: bool,
}

impl Default for BalanceWeight {
    fn default() -> Self {
        Self::new()
    }
}

impl BalanceWeight {
    pub fn new() -> Self {
        Self {
            clip_ok: true,
            adhesive_ok: true,
            position_ok: true,
            gram_ok: true,
            secure_ok: true,
        }
    }

    pub fn attachment_ok(&self) -> bool {
        self.clip_ok && self.adhesive_ok && self.secure_ok
    }

    pub fn calibration_ok(&self) -> bool {
        self.position_ok && self.gram_ok
    }

    pub fn all_ok(&self) -> bool {
        self.attachment_ok() && self.calibration_ok()
    }

    pub fn needs_rebalance(&self) -> bool {
        !self.position_ok || !self.secure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.secure_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attachment() {
        let c = BalanceWeight::new();
        assert!(c.attachment_ok());
    }

    #[test]
    fn test_calibration() {
        let c = BalanceWeight::new();
        assert!(c.calibration_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BalanceWeight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebalance() {
        let c = BalanceWeight::new();
        assert!(!c.needs_rebalance());
    }

    #[test]
    fn test_secure() {
        let mut c = BalanceWeight::new();
        c.secure_ok = false;
        assert!(c.needs_rebalance());
    }

    #[test]
    fn test_health() {
        let c = BalanceWeight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
