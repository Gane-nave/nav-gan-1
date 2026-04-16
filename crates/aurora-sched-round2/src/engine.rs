/// sched round2: add, next, remove, reset, log
/// Phase 2349

#[derive(Debug, Clone)]
pub struct SchedRound2 {
    pub add_ok: bool,
    pub next_ok: bool,
    pub remove_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedRound2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedRound2 {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            next_ok: true,
            remove_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.next_ok && self.remove_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.next_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.add_ok {
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
        let c = SchedRound2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedRound2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedRound2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedRound2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedRound2::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedRound2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
