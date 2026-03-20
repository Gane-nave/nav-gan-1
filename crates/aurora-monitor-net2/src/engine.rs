/// monitor net: sample, bandwidth, latency, error, log
/// Phase 1575

#[derive(Debug, Clone)]
pub struct MonitorNet2 {
    pub sample_ok: bool,
    pub bandwidth_ok: bool,
    pub latency_ok: bool,
    pub error_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorNet2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorNet2 {
    pub fn new() -> Self {
        Self {
            sample_ok: true,
            bandwidth_ok: true,
            latency_ok: true,
            error_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sample_ok && self.bandwidth_ok && self.latency_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.error_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sample_ok || !self.bandwidth_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sample_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MonitorNet2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorNet2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorNet2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorNet2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorNet2::new();
        c.sample_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorNet2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
