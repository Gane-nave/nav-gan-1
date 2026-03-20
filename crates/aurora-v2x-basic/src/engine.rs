/// v2x basic: broadcast, receive, decode, verify, relay
/// Phase 1122

#[derive(Debug, Clone)]
pub struct V2xBasic {
    pub broadcast_ok: bool,
    pub receive_ok: bool,
    pub decode_ok: bool,
    pub verify_ok: bool,
    pub relay_ok: bool,
}

impl Default for V2xBasic {
    fn default() -> Self {
        Self::new()
    }
}

impl V2xBasic {
    pub fn new() -> Self {
        Self {
            broadcast_ok: true,
            receive_ok: true,
            decode_ok: true,
            verify_ok: true,
            relay_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.broadcast_ok && self.receive_ok && self.decode_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.relay_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.broadcast_ok || !self.receive_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.broadcast_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = V2xBasic::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = V2xBasic::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2xBasic::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = V2xBasic::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = V2xBasic::new();
        c.broadcast_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = V2xBasic::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
