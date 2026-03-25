/// Hood latch: primary/secondary, cable, striker
/// Phase 553

#[derive(Debug, Clone)]
pub struct HoodLatch {
    pub primary_ok: bool,
    pub secondary_ok: bool,
    pub cable_ok: bool,
    pub striker_ok: bool,
    pub lubricated: bool,
}

impl Default for HoodLatch {
    fn default() -> Self {
        Self::new()
    }
}

impl HoodLatch {
    pub fn new() -> Self {
        Self {
            primary_ok: true,
            secondary_ok: true,
            cable_ok: true,
            striker_ok: true,
            lubricated: true,
        }
    }

    pub fn latch_ok(&self) -> bool {
        self.primary_ok && self.secondary_ok
    }

    pub fn mechanism_ok(&self) -> bool {
        self.cable_ok && self.striker_ok
    }

    pub fn all_ok(&self) -> bool {
        self.latch_ok() && self.mechanism_ok() && self.lubricated
    }

    pub fn needs_service(&self) -> bool {
        !self.primary_ok || !self.cable_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.primary_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latch() {
        let c = HoodLatch::new();
        assert!(c.latch_ok());
    }

    #[test]
    fn test_mechanism() {
        let c = HoodLatch::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HoodLatch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HoodLatch::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_primary() {
        let mut c = HoodLatch::new();
        c.primary_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HoodLatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
