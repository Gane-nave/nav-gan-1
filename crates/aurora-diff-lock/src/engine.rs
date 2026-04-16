/// diff lock: engage, disengage, sense, distribute, report
/// Phase 1218

#[derive(Debug, Clone)]
pub struct DiffLock {
    pub engage_ok: bool,
    pub disengage_ok: bool,
    pub sense_ok: bool,
    pub distribute_ok: bool,
    pub report_ok: bool,
}

impl Default for DiffLock {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffLock {
    pub fn new() -> Self {
        Self {
            engage_ok: true,
            disengage_ok: true,
            sense_ok: true,
            distribute_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.engage_ok && self.disengage_ok && self.sense_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.distribute_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.engage_ok || !self.disengage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.engage_ok {
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
        let c = DiffLock::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DiffLock::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DiffLock::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DiffLock::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DiffLock::new();
        c.engage_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DiffLock::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
