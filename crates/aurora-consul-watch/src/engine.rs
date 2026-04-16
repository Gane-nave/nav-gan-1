/// aurora-consul-watch: consul watch
/// Phase 2607

#[derive(Debug, Clone)]
pub struct ConsulWatch {
    pub subscribe_ok: bool,
    pub cancel_ok: bool,
    pub filter_ok: bool,
    pub event_ok: bool,
    pub notify_ok: bool,
}

impl Default for ConsulWatch {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsulWatch {
    pub fn new() -> Self {
        Self {
            subscribe_ok: true,
            cancel_ok: true,
            filter_ok: true,
            event_ok: true,
            notify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.subscribe_ok && self.cancel_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.event_ok && self.notify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.subscribe_ok || !self.cancel_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.subscribe_ok {
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
        let c = ConsulWatch::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ConsulWatch::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ConsulWatch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ConsulWatch::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ConsulWatch::new();
        c.subscribe_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ConsulWatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ConsulWatch::default();
        assert!(c.all_ok());
    }
}
