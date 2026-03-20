/// Blue-green deployment: switch, drain, verify, cutover, revert
/// Phase 1077

#[derive(Debug, Clone)]
pub struct BlueGreen {
    pub switch_ok: bool,
    pub drain_ok: bool,
    pub verify_ok: bool,
    pub cutover_ok: bool,
    pub revert_ok: bool,
}

impl Default for BlueGreen {
    fn default() -> Self {
        Self::new()
    }
}

impl BlueGreen {
    pub fn new() -> Self {
        Self {
            switch_ok: true,
            drain_ok: true,
            verify_ok: true,
            cutover_ok: true,
            revert_ok: true,
        }
    }

    pub fn transition_ok(&self) -> bool {
        self.switch_ok && self.drain_ok && self.cutover_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.verify_ok && self.revert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.transition_ok() && self.safety_ok()
    }

    pub fn needs_revert(&self) -> bool {
        !self.cutover_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.switch_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition() {
        let c = BlueGreen::new();
        assert!(c.transition_ok());
    }

    #[test]
    fn test_safety() {
        let c = BlueGreen::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BlueGreen::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_revert() {
        let c = BlueGreen::new();
        assert!(!c.needs_revert());
    }

    #[test]
    fn test_cutover() {
        let mut c = BlueGreen::new();
        c.cutover_ok = false;
        assert!(c.needs_revert());
    }

    #[test]
    fn test_health() {
        let c = BlueGreen::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
