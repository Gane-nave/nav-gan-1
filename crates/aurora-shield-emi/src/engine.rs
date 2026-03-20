/// shield emi: detect, filter, suppress, monitor, log
/// Phase 1370

#[derive(Debug, Clone)]
pub struct ShieldEmi {
    pub detect_ok: bool,
    pub filter_ok: bool,
    pub suppress_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for ShieldEmi {
    fn default() -> Self {
        Self::new()
    }
}

impl ShieldEmi {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            filter_ok: true,
            suppress_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.filter_ok && self.suppress_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ShieldEmi::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ShieldEmi::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ShieldEmi::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ShieldEmi::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ShieldEmi::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ShieldEmi::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
