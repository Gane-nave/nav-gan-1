/// stream throttle: limit, burst, window, reset, log
/// Phase 1930

#[derive(Debug, Clone)]
pub struct StreamThrottle {
    pub limit_ok: bool,
    pub burst_ok: bool,
    pub window_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for StreamThrottle {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamThrottle {
    pub fn new() -> Self {
        Self {
            limit_ok: true,
            burst_ok: true,
            window_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.limit_ok && self.burst_ok && self.window_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.limit_ok || !self.burst_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.limit_ok {
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
        let c = StreamThrottle::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StreamThrottle::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamThrottle::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StreamThrottle::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StreamThrottle::new();
        c.limit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StreamThrottle::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
