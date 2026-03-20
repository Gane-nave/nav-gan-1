/// Door lock: actuator, latch, key cylinder, remote
/// Phase 680

#[derive(Debug, Clone)]
pub struct DoorLock {
    pub actuator_ok: bool,
    pub latch_ok: bool,
    pub key_cyl_ok: bool,
    pub remote_ok: bool,
    pub child_lock_ok: bool,
}

impl Default for DoorLock {
    fn default() -> Self {
        Self::new()
    }
}

impl DoorLock {
    pub fn new() -> Self {
        Self {
            actuator_ok: true,
            latch_ok: true,
            key_cyl_ok: true,
            remote_ok: true,
            child_lock_ok: true,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.latch_ok && self.key_cyl_ok
    }

    pub fn electronic_ok(&self) -> bool {
        self.actuator_ok && self.remote_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.electronic_ok() && self.child_lock_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.actuator_ok || !self.latch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.latch_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = DoorLock::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_electronic() {
        let c = DoorLock::new();
        assert!(c.electronic_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DoorLock::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = DoorLock::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_latch() {
        let mut c = DoorLock::new();
        c.latch_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = DoorLock::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
