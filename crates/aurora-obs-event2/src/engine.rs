/// obs event2: emit, collect, correlate, alert, log
/// Phase 2151

#[derive(Debug, Clone)]
pub struct ObsEvent2 {
    pub emit_ok: bool,
    pub collect_ok: bool,
    pub correlate_ok: bool,
    pub alert_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsEvent2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsEvent2 {
    pub fn new() -> Self {
        Self {
            emit_ok: true,
            collect_ok: true,
            correlate_ok: true,
            alert_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.emit_ok && self.collect_ok && self.correlate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.emit_ok || !self.collect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.emit_ok {
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
        let c = ObsEvent2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsEvent2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsEvent2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsEvent2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsEvent2::new();
        c.emit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsEvent2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
