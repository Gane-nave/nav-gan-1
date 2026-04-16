/// Smart parking: detect, reserve, guide, pay, exit
/// Phase 1103

#[derive(Debug, Clone)]
pub struct SmartPark {
    pub detect_ok: bool,
    pub reserve_ok: bool,
    pub guide_ok: bool,
    pub pay_ok: bool,
    pub exit_ok: bool,
}

impl Default for SmartPark {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartPark {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            reserve_ok: true,
            guide_ok: true,
            pay_ok: true,
            exit_ok: true,
        }
    }

    pub fn finding_ok(&self) -> bool {
        self.detect_ok && self.reserve_ok && self.guide_ok
    }

    pub fn transaction_ok(&self) -> bool {
        self.pay_ok && self.exit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.finding_ok() && self.transaction_ok()
    }

    pub fn needs_refresh(&self) -> bool {
        !self.detect_ok || !self.reserve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finding() {
        let c = SmartPark::new();
        assert!(c.finding_ok());
    }

    #[test]
    fn test_transaction() {
        let c = SmartPark::new();
        assert!(c.transaction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SmartPark::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refresh() {
        let c = SmartPark::new();
        assert!(!c.needs_refresh());
    }

    #[test]
    fn test_detect() {
        let mut c = SmartPark::new();
        c.detect_ok = false;
        assert!(c.needs_refresh());
    }

    #[test]
    fn test_health() {
        let c = SmartPark::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
