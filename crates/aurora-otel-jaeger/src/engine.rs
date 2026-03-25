/// otel jaeger: connect, send, batch, flush, log
/// Phase 1764

#[derive(Debug, Clone)]
pub struct OtelJaeger {
    pub connect_ok: bool,
    pub send_ok: bool,
    pub batch_ok: bool,
    pub flush_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelJaeger {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelJaeger {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            send_ok: true,
            batch_ok: true,
            flush_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.send_ok && self.batch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flush_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.send_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = OtelJaeger::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelJaeger::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelJaeger::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelJaeger::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelJaeger::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelJaeger::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
