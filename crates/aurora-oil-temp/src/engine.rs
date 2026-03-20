/// Oil temperature: engine, trans, diff, cooler, warning
/// Phase 953

#[derive(Debug, Clone)]
pub struct OilTemp {
    pub engine_ok: bool,
    pub trans_ok: bool,
    pub diff_ok: bool,
    pub cooler_ok: bool,
    pub warning_ok: bool,
}

impl Default for OilTemp {
    fn default() -> Self {
        Self::new()
    }
}

impl OilTemp {
    pub fn new() -> Self {
        Self {
            engine_ok: true,
            trans_ok: true,
            diff_ok: true,
            cooler_ok: true,
            warning_ok: true,
        }
    }

    pub fn monitoring_ok(&self) -> bool {
        self.engine_ok && self.trans_ok && self.diff_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.cooler_ok && self.warning_ok
    }

    pub fn all_ok(&self) -> bool {
        self.monitoring_ok() && self.protection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.cooler_ok || !self.engine_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.engine_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring() {
        let c = OilTemp::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_protection() {
        let c = OilTemp::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilTemp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = OilTemp::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_engine() {
        let mut c = OilTemp::new();
        c.engine_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = OilTemp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
