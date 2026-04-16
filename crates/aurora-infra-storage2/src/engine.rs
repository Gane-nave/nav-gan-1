/// infra storage2: create, configure, monitor, scale, log
/// Phase 2141

#[derive(Debug, Clone)]
pub struct InfraStorage2 {
    pub create_ok: bool,
    pub configure_ok: bool,
    pub monitor_ok: bool,
    pub scale_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraStorage2 {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraStorage2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            configure_ok: true,
            monitor_ok: true,
            scale_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.configure_ok && self.monitor_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.scale_ok && self.log_ok
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
        let c = InfraStorage2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraStorage2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraStorage2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraStorage2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraStorage2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraStorage2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
