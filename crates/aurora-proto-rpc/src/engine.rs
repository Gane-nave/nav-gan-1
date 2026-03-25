/// proto rpc: call, respond, stream, cancel, log
/// Phase 2019

#[derive(Debug, Clone)]
pub struct ProtoRpc {
    pub call_ok: bool,
    pub respond_ok: bool,
    pub stream_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for ProtoRpc {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoRpc {
    pub fn new() -> Self {
        Self {
            call_ok: true,
            respond_ok: true,
            stream_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.call_ok && self.respond_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.call_ok || !self.respond_ok
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
        let c = ProtoRpc::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ProtoRpc::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProtoRpc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ProtoRpc::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ProtoRpc::new();
        c.call_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ProtoRpc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
