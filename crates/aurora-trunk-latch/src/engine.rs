/// Trunk latch: electric release, emergency release, striker alignment
/// Phase 415

#[derive(Debug, Clone)]
pub struct TrunkLatch {
    pub engaged: bool,
    pub electric_ok: bool,
    pub emergency_ok: bool,
    pub striker_ok: bool,
    pub actuator_ok: bool,
}

impl Default for TrunkLatch {
    fn default() -> Self {
        Self::new()
    }
}

impl TrunkLatch {
    pub fn new() -> Self {
        Self {
            engaged: true,
            electric_ok: true,
            emergency_ok: true,
            striker_ok: true,
            actuator_ok: true,
        }
    }

    pub fn secure(&self) -> bool {
        self.engaged && self.striker_ok
    }

    pub fn all_ok(&self) -> bool {
        self.secure() && self.electric_ok && self.emergency_ok && self.actuator_ok
    }

    pub fn needs_repair(&self) -> bool {
        !self.actuator_ok || !self.electric_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.emergency_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.emergency_ok {
            return 0.0;
        }
        if !self.actuator_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure() {
        let t = TrunkLatch::new();
        assert!(t.secure());
    }

    #[test]
    fn test_all_ok() {
        let t = TrunkLatch::new();
        assert!(t.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let t = TrunkLatch::new();
        assert!(!t.needs_repair());
    }

    #[test]
    fn test_safety() {
        let t = TrunkLatch::new();
        assert!(t.safety_ok());
    }

    #[test]
    fn test_bad_actuator() {
        let mut t = TrunkLatch::new();
        t.actuator_ok = false;
        assert!(t.needs_repair());
    }

    #[test]
    fn test_health() {
        let t = TrunkLatch::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
