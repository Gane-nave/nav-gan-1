/// monitor disk: sample, usage, iops, latency, log
/// Phase 1574

#[derive(Debug, Clone)]
pub struct MonitorDisk {
    pub sample_ok: bool,
    pub usage_ok: bool,
    pub iops_ok: bool,
    pub latency_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorDisk {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorDisk {
    pub fn new() -> Self {
        Self {
            sample_ok: true,
            usage_ok: true,
            iops_ok: true,
            latency_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sample_ok && self.usage_ok && self.iops_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.latency_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sample_ok || !self.usage_ok
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
        let c = MonitorDisk::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorDisk::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorDisk::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorDisk::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorDisk::new();
        c.sample_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorDisk::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
