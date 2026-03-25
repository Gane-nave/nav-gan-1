/// Limited slip differential: clutch pack, torsen, helical
/// Phase 471

#[derive(Debug, Clone)]
pub struct LimitedSlip {
    pub bias_ratio: f64,
    pub clutch_ok: bool,
    pub preload_ok: bool,
    pub oil_ok: bool,
    pub effective: bool,
}

impl Default for LimitedSlip {
    fn default() -> Self {
        Self::new()
    }
}

impl LimitedSlip {
    pub fn new() -> Self {
        Self {
            bias_ratio: 2.5,
            clutch_ok: true,
            preload_ok: true,
            oil_ok: true,
            effective: true,
        }
    }

    pub fn locking_ok(&self) -> bool {
        self.effective && self.clutch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.clutch_ok && self.preload_ok && self.oil_ok && self.effective
    }

    pub fn needs_service(&self) -> bool {
        !self.oil_ok || !self.clutch_ok
    }

    pub fn traction_ok(&self) -> bool {
        self.effective && self.bias_ratio > 1.5
    }

    pub fn health_score(&self) -> f64 {
        if !self.clutch_ok {
            return 10.0;
        }
        if !self.oil_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locking() {
        let l = LimitedSlip::new();
        assert!(l.locking_ok());
    }

    #[test]
    fn test_all_ok() {
        let l = LimitedSlip::new();
        assert!(l.all_ok());
    }

    #[test]
    fn test_no_service() {
        let l = LimitedSlip::new();
        assert!(!l.needs_service());
    }

    #[test]
    fn test_traction() {
        let l = LimitedSlip::new();
        assert!(l.traction_ok());
    }

    #[test]
    fn test_worn_clutch() {
        let mut l = LimitedSlip::new();
        l.clutch_ok = false;
        assert!(l.needs_service());
    }

    #[test]
    fn test_health() {
        let l = LimitedSlip::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
