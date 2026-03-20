/// io rate: limit, burst, throttle, stats, log
/// Phase 1998

#[derive(Debug, Clone)]
pub struct IoRate {
    pub limit_ok: bool,
    pub burst_ok: bool,
    pub throttle_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for IoRate {
    fn default() -> Self {
        Self::new()
    }
}

impl IoRate {
    pub fn new() -> Self {
        Self {
            limit_ok: true,
            burst_ok: true,
            throttle_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.limit_ok && self.burst_ok && self.throttle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
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
        let c = IoRate::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoRate::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoRate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoRate::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoRate::new();
        c.limit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoRate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
