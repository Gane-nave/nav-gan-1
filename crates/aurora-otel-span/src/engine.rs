/// otel span: create, update, status, events, log
/// Phase 1761

#[derive(Debug, Clone)]
pub struct OtelSpan {
    pub create_ok: bool,
    pub update_ok: bool,
    pub status_ok: bool,
    pub events_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelSpan {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelSpan {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            update_ok: true,
            status_ok: true,
            events_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.update_ok && self.status_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.events_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.update_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = OtelSpan::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelSpan::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelSpan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelSpan::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelSpan::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelSpan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
