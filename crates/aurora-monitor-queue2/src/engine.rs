/// monitor queue: depth, rate, latency, overflow, log
/// Phase 1581

#[derive(Debug, Clone)]
pub struct MonitorQueue2 {
    pub depth_ok: bool,
    pub rate_ok: bool,
    pub latency_ok: bool,
    pub overflow_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorQueue2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorQueue2 {
    pub fn new() -> Self {
        Self {
            depth_ok: true,
            rate_ok: true,
            latency_ok: true,
            overflow_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.depth_ok && self.rate_ok && self.latency_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.overflow_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.depth_ok || !self.rate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.depth_ok {
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
        let c = MonitorQueue2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorQueue2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorQueue2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorQueue2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorQueue2::new();
        c.depth_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorQueue2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
