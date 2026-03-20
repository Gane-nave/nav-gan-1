/// io retry2: attempt, backoff, reset, stats, log
/// Phase 1999

#[derive(Debug, Clone)]
pub struct IoRetry2 {
    pub attempt_ok: bool,
    pub backoff_ok: bool,
    pub reset_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for IoRetry2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IoRetry2 {
    pub fn new() -> Self {
        Self {
            attempt_ok: true,
            backoff_ok: true,
            reset_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.attempt_ok && self.backoff_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.attempt_ok || !self.backoff_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.attempt_ok {
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
        let c = IoRetry2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoRetry2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoRetry2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoRetry2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoRetry2::new();
        c.attempt_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoRetry2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
