/// io ring: submit, complete, cancel, stats, log
/// Phase 1993

#[derive(Debug, Clone)]
pub struct IoRing {
    pub submit_ok: bool,
    pub complete_ok: bool,
    pub cancel_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for IoRing {
    fn default() -> Self {
        Self::new()
    }
}

impl IoRing {
    pub fn new() -> Self {
        Self {
            submit_ok: true,
            complete_ok: true,
            cancel_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.submit_ok && self.complete_ok && self.cancel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.submit_ok || !self.complete_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.submit_ok {
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
        let c = IoRing::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoRing::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoRing::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoRing::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoRing::new();
        c.submit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoRing::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
