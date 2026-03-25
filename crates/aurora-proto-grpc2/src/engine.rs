/// proto grpc2: call, stream, metadata, cancel, log
/// Phase 2005

#[derive(Debug, Clone)]
pub struct ProtoGrpc2 {
    pub call_ok: bool,
    pub stream_ok: bool,
    pub metadata_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for ProtoGrpc2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoGrpc2 {
    pub fn new() -> Self {
        Self {
            call_ok: true,
            stream_ok: true,
            metadata_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.call_ok && self.stream_ok && self.metadata_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.call_ok || !self.stream_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.call_ok {
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
        let c = ProtoGrpc2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ProtoGrpc2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProtoGrpc2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ProtoGrpc2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ProtoGrpc2::new();
        c.call_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ProtoGrpc2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
