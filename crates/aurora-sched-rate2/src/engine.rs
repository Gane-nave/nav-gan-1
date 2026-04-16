/// sched rate2: configure, check, throttle, release, log
/// Phase 2356

#[derive(Debug, Clone)]
pub struct SchedRate2 {
    pub configure_ok: bool,
    pub check_ok: bool,
    pub throttle_ok: bool,
    pub release_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedRate2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedRate2 {
    pub fn new() -> Self {
        Self {
            configure_ok: true,
            check_ok: true,
            throttle_ok: true,
            release_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.configure_ok && self.check_ok && self.throttle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.release_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.configure_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.configure_ok {
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
        let c = SchedRate2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedRate2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedRate2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedRate2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedRate2::new();
        c.configure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedRate2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
