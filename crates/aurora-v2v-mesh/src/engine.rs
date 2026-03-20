/// v2v mesh: peer, discover, handshake, exchange, sync
/// Phase 1124

#[derive(Debug, Clone)]
pub struct V2vMesh {
    pub peer_ok: bool,
    pub discover_ok: bool,
    pub handshake_ok: bool,
    pub exchange_ok: bool,
    pub sync_ok: bool,
}

impl Default for V2vMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl V2vMesh {
    pub fn new() -> Self {
        Self {
            peer_ok: true,
            discover_ok: true,
            handshake_ok: true,
            exchange_ok: true,
            sync_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.peer_ok && self.discover_ok && self.handshake_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.exchange_ok && self.sync_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.peer_ok || !self.discover_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.peer_ok {
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
        let c = V2vMesh::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = V2vMesh::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2vMesh::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = V2vMesh::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = V2vMesh::new();
        c.peer_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = V2vMesh::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
