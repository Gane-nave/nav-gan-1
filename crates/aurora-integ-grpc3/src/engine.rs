/// integ grpc3: connect, call, stream, retry, log
/// Phase 2240

#[derive(Debug, Clone)]
pub struct IntegGrpc3 {
    pub connect_ok: bool,
    pub call_ok: bool,
    pub stream_ok: bool,
    pub retry_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegGrpc3 {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegGrpc3 {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            call_ok: true,
            stream_ok: true,
            retry_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.call_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retry_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.call_ok
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
        let c = IntegGrpc3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegGrpc3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegGrpc3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegGrpc3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegGrpc3::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegGrpc3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
