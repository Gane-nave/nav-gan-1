/// io timeout2: set, check, extend, cancel, log
/// Phase 2000

#[derive(Debug, Clone)]
pub struct IoTimeout2 {
    pub set_ok: bool,
    pub check_ok: bool,
    pub extend_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for IoTimeout2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IoTimeout2 {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            check_ok: true,
            extend_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.check_ok && self.extend_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
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
        let c = IoTimeout2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoTimeout2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoTimeout2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoTimeout2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoTimeout2::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoTimeout2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
