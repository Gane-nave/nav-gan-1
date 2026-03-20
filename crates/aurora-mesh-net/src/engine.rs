/// Mesh network: peer, relay, route, heal, discover
/// Phase 983

#[derive(Debug, Clone)]
pub struct MeshNet {
    pub peer_ok: bool,
    pub relay_ok: bool,
    pub route_ok: bool,
    pub heal_ok: bool,
    pub discover_ok: bool,
}

impl Default for MeshNet {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshNet {
    pub fn new() -> Self {
        Self {
            peer_ok: true,
            relay_ok: true,
            route_ok: true,
            heal_ok: true,
            discover_ok: true,
        }
    }

    pub fn topology_ok(&self) -> bool {
        self.peer_ok && self.relay_ok && self.discover_ok
    }

    pub fn resilience_ok(&self) -> bool {
        self.route_ok && self.heal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.topology_ok() && self.resilience_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.peer_ok || !self.discover_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.peer_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology() {
        let c = MeshNet::new();
        assert!(c.topology_ok());
    }

    #[test]
    fn test_resilience() {
        let c = MeshNet::new();
        assert!(c.resilience_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MeshNet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = MeshNet::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_peer() {
        let mut c = MeshNet::new();
        c.peer_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = MeshNet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
