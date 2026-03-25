/// monitor cpu: sample, average, spike, throttle, log
/// Phase 1572

#[derive(Debug, Clone)]
pub struct MonitorCpu {
    pub sample_ok: bool,
    pub average_ok: bool,
    pub spike_ok: bool,
    pub throttle_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorCpu {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorCpu {
    pub fn new() -> Self {
        Self {
            sample_ok: true,
            average_ok: true,
            spike_ok: true,
            throttle_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sample_ok && self.average_ok && self.spike_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.throttle_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sample_ok || !self.average_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sample_ok {
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
        let c = MonitorCpu::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorCpu::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorCpu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorCpu::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorCpu::new();
        c.sample_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorCpu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
