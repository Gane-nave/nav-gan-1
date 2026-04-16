/// io batch: collect, flush, timeout, stats, log
/// Phase 2002

#[derive(Debug, Clone)]
pub struct IoBatch {
    pub collect_ok: bool,
    pub flush_ok: bool,
    pub timeout_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for IoBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl IoBatch {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            flush_ok: true,
            timeout_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.flush_ok && self.timeout_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.flush_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
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
        let c = IoBatch::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoBatch::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoBatch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoBatch::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoBatch::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoBatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
