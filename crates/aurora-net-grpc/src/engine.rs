/// net grpc: define, serve, call, stream, log
/// Phase 1539

#[derive(Debug, Clone)]
pub struct NetGrpc {
    pub define_ok: bool,
    pub serve_ok: bool,
    pub call_ok: bool,
    pub stream_ok: bool,
    pub log_ok: bool,
}

impl Default for NetGrpc {
    fn default() -> Self {
        Self::new()
    }
}

impl NetGrpc {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            serve_ok: true,
            call_ok: true,
            stream_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.serve_ok && self.call_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stream_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.serve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = NetGrpc::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetGrpc::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetGrpc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetGrpc::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetGrpc::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetGrpc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
