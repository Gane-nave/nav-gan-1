/// Thread locking: Loctite, prevailing torque, chemical locking
/// Phase 403

#[derive(Debug, Clone)]
pub struct ThreadLock {
    pub applied: bool,
    pub strength: u8,
    pub cured: bool,
    pub removable: bool,
    pub torque_ok: bool,
}

impl Default for ThreadLock {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadLock {
    pub fn new() -> Self {
        Self {
            applied: true,
            strength: 2,
            cured: true,
            removable: true,
            torque_ok: true,
        }
    }

    pub fn effective(&self) -> bool {
        self.applied && self.cured
    }

    pub fn high_strength(&self) -> bool {
        self.strength >= 3
    }

    pub fn serviceable(&self) -> bool {
        self.removable
    }

    pub fn needs_reapply(&self) -> bool {
        !self.applied || !self.cured
    }

    pub fn health_score(&self) -> f64 {
        if !self.applied {
            return 0.0;
        }
        if !self.cured {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective() {
        let t = ThreadLock::new();
        assert!(t.effective());
    }

    #[test]
    fn test_not_high() {
        let t = ThreadLock::new();
        assert!(!t.high_strength());
    }

    #[test]
    fn test_serviceable() {
        let t = ThreadLock::new();
        assert!(t.serviceable());
    }

    #[test]
    fn test_no_reapply() {
        let t = ThreadLock::new();
        assert!(!t.needs_reapply());
    }

    #[test]
    fn test_missing() {
        let mut t = ThreadLock::new();
        t.applied = false;
        assert!(t.needs_reapply());
    }

    #[test]
    fn test_health() {
        let t = ThreadLock::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
