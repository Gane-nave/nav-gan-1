/// monitor response: status, latency, size, error, log
/// Phase 1585

#[derive(Debug, Clone)]
pub struct MonitorResponse {
    pub status_ok: bool,
    pub latency_ok: bool,
    pub size_ok: bool,
    pub error_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorResponse {
    pub fn new() -> Self {
        Self {
            status_ok: true,
            latency_ok: true,
            size_ok: true,
            error_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.status_ok && self.latency_ok && self.size_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.error_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.status_ok || !self.latency_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.status_ok {
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
        let c = MonitorResponse::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorResponse::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorResponse::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorResponse::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorResponse::new();
        c.status_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorResponse::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
