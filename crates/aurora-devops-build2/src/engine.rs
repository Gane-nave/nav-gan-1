/// devops build2: compile, test, package, publish, log
/// Phase 2167

#[derive(Debug, Clone)]
pub struct DevopsBuild2 {
    pub compile_ok: bool,
    pub test_ok: bool,
    pub package_ok: bool,
    pub publish_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsBuild2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsBuild2 {
    pub fn new() -> Self {
        Self {
            compile_ok: true,
            test_ok: true,
            package_ok: true,
            publish_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compile_ok && self.test_ok && self.package_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.publish_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.compile_ok || !self.test_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compile_ok {
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
        let c = DevopsBuild2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsBuild2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsBuild2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsBuild2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsBuild2::new();
        c.compile_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsBuild2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
