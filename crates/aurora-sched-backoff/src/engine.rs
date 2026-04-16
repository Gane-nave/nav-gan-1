/// sched backoff: retry, reset, config, status, log
/// Phase 1740

#[derive(Debug, Clone)]
pub struct SchedBackoff {
    pub retry_ok: bool,
    pub reset_ok: bool,
    pub config_ok: bool,
    pub status_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedBackoff {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedBackoff {
    pub fn new() -> Self {
        Self {
            retry_ok: true,
            reset_ok: true,
            config_ok: true,
            status_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.retry_ok && self.reset_ok && self.config_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.status_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.retry_ok || !self.reset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.retry_ok {
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
        let c = SchedBackoff::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedBackoff::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedBackoff::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedBackoff::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedBackoff::new();
        c.retry_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedBackoff::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
