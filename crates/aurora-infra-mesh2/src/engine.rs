/// infra mesh2: inject, configure, monitor, upgrade, log
/// Phase 2145

#[derive(Debug, Clone)]
pub struct InfraMesh2 {
    pub inject_ok: bool,
    pub configure_ok: bool,
    pub monitor_ok: bool,
    pub upgrade_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraMesh2 {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraMesh2 {
    pub fn new() -> Self {
        Self {
            inject_ok: true,
            configure_ok: true,
            monitor_ok: true,
            upgrade_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.inject_ok && self.configure_ok && self.monitor_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.upgrade_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.inject_ok || !self.configure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inject_ok {
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
        let c = InfraMesh2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraMesh2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraMesh2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraMesh2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraMesh2::new();
        c.inject_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraMesh2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
