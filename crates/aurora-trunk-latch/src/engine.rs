/// Trunk latch: actuator, switch, seal, strut
/// Phase 554

#[derive(Debug, Clone)]
pub struct TrunkLatch {
    pub actuator_ok: bool,
    pub switch_ok: bool,
    pub seal_ok: bool,
    pub strut_ok: bool,
    pub locked: bool,
}

impl Default for TrunkLatch {
    fn default() -> Self {
        Self::new()
    }
}

impl TrunkLatch {
    pub fn new() -> Self {
        Self {
            actuator_ok: true,
            switch_ok: true,
            seal_ok: true,
            strut_ok: true,
            locked: true,
        }
    }

    pub fn latch_ok(&self) -> bool {
        self.actuator_ok && self.switch_ok
    }

    pub fn support_ok(&self) -> bool {
        self.strut_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.latch_ok() && self.support_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.actuator_ok || !self.strut_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latch() {
        let c = TrunkLatch::new();
        assert!(c.latch_ok());
    }

    #[test]
    fn test_support() {
        let c = TrunkLatch::new();
        assert!(c.support_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrunkLatch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TrunkLatch::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_actuator() {
        let mut c = TrunkLatch::new();
        c.actuator_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TrunkLatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
