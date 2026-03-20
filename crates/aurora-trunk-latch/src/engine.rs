/// Trunk latch: striker, actuator, release, switch
/// Phase 684

#[derive(Debug, Clone)]
pub struct TrunkLatch {
    pub striker_ok: bool,
    pub actuator_ok: bool,
    pub release_ok: bool,
    pub switch_ok: bool,
    pub seal_ok: bool,
}

impl Default for TrunkLatch {
    fn default() -> Self {
        Self::new()
    }
}

impl TrunkLatch {
    pub fn new() -> Self {
        Self {
            striker_ok: true,
            actuator_ok: true,
            release_ok: true,
            switch_ok: true,
            seal_ok: true,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.striker_ok && self.release_ok
    }

    pub fn electronic_ok(&self) -> bool {
        self.actuator_ok && self.switch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.electronic_ok() && self.seal_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.striker_ok || !self.actuator_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.striker_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = TrunkLatch::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_electronic() {
        let c = TrunkLatch::new();
        assert!(c.electronic_ok());
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
    fn test_striker() {
        let mut c = TrunkLatch::new();
        c.striker_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TrunkLatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
