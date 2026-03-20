/// lane change: check, signal, execute, abort, confirm
/// Phase 1321

#[derive(Debug, Clone)]
pub struct LaneChange {
    pub check_ok: bool,
    pub signal_ok: bool,
    pub execute_ok: bool,
    pub abort_ok: bool,
    pub confirm_ok: bool,
}

impl Default for LaneChange {
    fn default() -> Self {
        Self::new()
    }
}

impl LaneChange {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            signal_ok: true,
            execute_ok: true,
            abort_ok: true,
            confirm_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.signal_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.abort_ok && self.confirm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.signal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = LaneChange::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LaneChange::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LaneChange::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LaneChange::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LaneChange::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LaneChange::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
