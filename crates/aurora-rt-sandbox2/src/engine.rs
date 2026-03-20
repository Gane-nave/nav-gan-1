/// rt sandbox2: create, restrict, execute, destroy, log
/// Phase 2340

#[derive(Debug, Clone)]
pub struct RtSandbox2 {
    pub create_ok: bool,
    pub restrict_ok: bool,
    pub execute_ok: bool,
    pub destroy_ok: bool,
    pub log_ok: bool,
}

impl Default for RtSandbox2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtSandbox2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            restrict_ok: true,
            execute_ok: true,
            destroy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.restrict_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.destroy_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.restrict_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = RtSandbox2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtSandbox2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtSandbox2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtSandbox2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtSandbox2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtSandbox2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
