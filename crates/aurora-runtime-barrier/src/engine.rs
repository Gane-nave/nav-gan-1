/// runtime barrier: wait, reset, leader, count, log
/// Phase 1802

#[derive(Debug, Clone)]
pub struct RuntimeBarrier {
    pub wait_ok: bool,
    pub reset_ok: bool,
    pub leader_ok: bool,
    pub count_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeBarrier {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeBarrier {
    pub fn new() -> Self {
        Self {
            wait_ok: true,
            reset_ok: true,
            leader_ok: true,
            count_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.wait_ok && self.reset_ok && self.leader_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.count_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.wait_ok || !self.reset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wait_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = RuntimeBarrier::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeBarrier::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeBarrier::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeBarrier::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeBarrier::new();
        c.wait_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeBarrier::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
