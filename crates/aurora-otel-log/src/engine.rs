/// otel log: emit, filter, format, export, log
/// Phase 1754

#[derive(Debug, Clone)]
pub struct OtelLog {
    pub emit_ok: bool,
    pub filter_ok: bool,
    pub format_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelLog {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelLog {
    pub fn new() -> Self {
        Self {
            emit_ok: true,
            filter_ok: true,
            format_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.emit_ok && self.filter_ok && self.format_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.emit_ok || !self.filter_ok
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
        let c = OtelLog::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelLog::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelLog::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelLog::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelLog::new();
        c.emit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelLog::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
