/// monitor request: count, latency, error, throughput, log
/// Phase 1584

#[derive(Debug, Clone)]
pub struct MonitorRequest {
    pub count_ok: bool,
    pub latency_ok: bool,
    pub error_ok: bool,
    pub throughput_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorRequest {
    pub fn new() -> Self {
        Self {
            count_ok: true,
            latency_ok: true,
            error_ok: true,
            throughput_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.count_ok && self.latency_ok && self.error_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.throughput_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.count_ok || !self.latency_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.count_ok {
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
        let c = MonitorRequest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorRequest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorRequest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorRequest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorRequest::new();
        c.count_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorRequest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
