/// infra nat: create, configure, monitor, rotate, log
/// Phase 2137

#[derive(Debug, Clone)]
pub struct InfraNat {
    pub create_ok: bool,
    pub configure_ok: bool,
    pub monitor_ok: bool,
    pub rotate_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraNat {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraNat {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            configure_ok: true,
            monitor_ok: true,
            rotate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.configure_ok && self.monitor_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rotate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.configure_ok
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
        let c = InfraNat::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraNat::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraNat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraNat::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraNat::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraNat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
