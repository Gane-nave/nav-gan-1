/// dash module: gauge, indicator, warning, dim, check
/// Phase 1278

#[derive(Debug, Clone)]
pub struct DashModule {
    pub gauge_ok: bool,
    pub indicator_ok: bool,
    pub warning_ok: bool,
    pub dim_ok: bool,
    pub check_ok: bool,
}

impl Default for DashModule {
    fn default() -> Self {
        Self::new()
    }
}

impl DashModule {
    pub fn new() -> Self {
        Self {
            gauge_ok: true,
            indicator_ok: true,
            warning_ok: true,
            dim_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.gauge_ok && self.indicator_ok && self.warning_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dim_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.gauge_ok || !self.indicator_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gauge_ok {
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
        let c = DashModule::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashModule::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashModule::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashModule::new();
        c.gauge_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
