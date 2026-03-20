/// Latch lock: door latch, safety catch, child lock, central locking
/// Phase 410

#[derive(Debug, Clone)]
pub struct LatchLock {
    pub engaged: bool,
    pub secondary_catch: bool,
    pub child_lock: bool,
    pub actuator_ok: bool,
    pub striker_aligned: bool,
}

impl Default for LatchLock {
    fn default() -> Self {
        Self::new()
    }
}

impl LatchLock {
    pub fn new() -> Self {
        Self {
            engaged: true,
            secondary_catch: true,
            child_lock: false,
            actuator_ok: true,
            striker_aligned: true,
        }
    }

    pub fn secure(&self) -> bool {
        self.engaged && self.secondary_catch
    }

    pub fn all_ok(&self) -> bool {
        self.secure() && self.actuator_ok && self.striker_aligned
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.striker_aligned
    }

    pub fn needs_repair(&self) -> bool {
        !self.actuator_ok || !self.secondary_catch
    }

    pub fn health_score(&self) -> f64 {
        if !self.secondary_catch {
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
        let l = LatchLock::new();
        assert!(l.secure());
    }

    #[test]
    fn test_all_ok() {
        let l = LatchLock::new();
        assert!(l.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let l = LatchLock::new();
        assert!(!l.needs_adjustment());
    }

    #[test]
    fn test_no_repair() {
        let l = LatchLock::new();
        assert!(!l.needs_repair());
    }

    #[test]
    fn test_bad_catch() {
        let mut l = LatchLock::new();
        l.secondary_catch = false;
        assert!(l.needs_repair());
    }

    #[test]
    fn test_health() {
        let l = LatchLock::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
