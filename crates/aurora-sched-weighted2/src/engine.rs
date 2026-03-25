/// sched weighted2: add, next, adjust, remove, log
/// Phase 2350

#[derive(Debug, Clone)]
pub struct SchedWeighted2 {
    pub add_ok: bool,
    pub next_ok: bool,
    pub adjust_ok: bool,
    pub remove_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedWeighted2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedWeighted2 {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            next_ok: true,
            adjust_ok: true,
            remove_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.next_ok && self.adjust_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.remove_ok && self.log_ok
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
        let c = SchedWeighted2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedWeighted2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedWeighted2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedWeighted2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedWeighted2::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedWeighted2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
