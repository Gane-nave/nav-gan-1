/// platform embedded: detect, configure, optimize, monitor, log
/// Phase 1812

#[derive(Debug, Clone)]
pub struct PlatformEmbedded {
    pub detect_ok: bool,
    pub configure_ok: bool,
    pub optimize_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for PlatformEmbedded {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformEmbedded {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            configure_ok: true,
            optimize_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.configure_ok && self.optimize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.configure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = PlatformEmbedded::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PlatformEmbedded::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PlatformEmbedded::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PlatformEmbedded::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PlatformEmbedded::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PlatformEmbedded::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
