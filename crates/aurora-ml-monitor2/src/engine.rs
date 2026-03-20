/// ml monitor: drift, accuracy, latency, resource, log
/// Phase 1470

#[derive(Debug, Clone)]
pub struct MlMonitor2 {
    pub drift_ok: bool,
    pub accuracy_ok: bool,
    pub latency_ok: bool,
    pub resource_ok: bool,
    pub log_ok: bool,
}

impl Default for MlMonitor2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MlMonitor2 {
    pub fn new() -> Self {
        Self {
            drift_ok: true,
            accuracy_ok: true,
            latency_ok: true,
            resource_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.drift_ok && self.accuracy_ok && self.latency_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resource_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.drift_ok || !self.accuracy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.drift_ok {
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
        let c = MlMonitor2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlMonitor2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlMonitor2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlMonitor2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlMonitor2::new();
        c.drift_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlMonitor2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
