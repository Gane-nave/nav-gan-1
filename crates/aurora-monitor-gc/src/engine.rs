/// monitor gc: pause, collect, promote, compact, log
/// Phase 1578

#[derive(Debug, Clone)]
pub struct MonitorGc {
    pub pause_ok: bool,
    pub collect_ok: bool,
    pub promote_ok: bool,
    pub compact_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorGc {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorGc {
    pub fn new() -> Self {
        Self {
            pause_ok: true,
            collect_ok: true,
            promote_ok: true,
            compact_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.pause_ok && self.collect_ok && self.promote_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compact_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.pause_ok || !self.collect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pause_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MonitorGc::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorGc::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorGc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorGc::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorGc::new();
        c.pause_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorGc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
