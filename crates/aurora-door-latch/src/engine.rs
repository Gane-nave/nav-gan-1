/// Door latch: striker, actuator, child lock, ajar switch
/// Phase 536

#[derive(Debug, Clone)]
pub struct DoorLatch {
    pub striker_ok: bool,
    pub actuator_ok: bool,
    pub child_lock: bool,
    pub ajar: bool,
    pub sealed: bool,
}

impl Default for DoorLatch {
    fn default() -> Self {
        Self::new()
    }
}

impl DoorLatch {
    pub fn new() -> Self {
        Self {
            striker_ok: true,
            actuator_ok: true,
            child_lock: false,
            ajar: false,
            sealed: true,
        }
    }

    pub fn latch_ok(&self) -> bool {
        self.striker_ok && self.actuator_ok
    }

    pub fn is_closed(&self) -> bool {
        !self.ajar && self.sealed
    }

    pub fn all_ok(&self) -> bool {
        self.latch_ok() && self.is_closed()
    }

    pub fn needs_service(&self) -> bool {
        !self.striker_ok || !self.actuator_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latch() {
        let c = DoorLatch::new();
        assert!(c.latch_ok());
    }

    #[test]
    fn test_closed() {
        let c = DoorLatch::new();
        assert!(c.is_closed());
    }

    #[test]
    fn test_all_ok() {
        let c = DoorLatch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = DoorLatch::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_actuator() {
        let mut c = DoorLatch::new();
        c.actuator_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = DoorLatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
