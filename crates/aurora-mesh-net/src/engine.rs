/// mesh network: join, route, forward, heal, leave
/// Phase 1138

#[derive(Debug, Clone)]
pub struct MeshNet {
    pub join_ok: bool,
    pub route_ok: bool,
    pub forward_ok: bool,
    pub heal_ok: bool,
    pub leave_ok: bool,
}

impl Default for MeshNet {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshNet {
    pub fn new() -> Self {
        Self {
            join_ok: true,
            route_ok: true,
            forward_ok: true,
            heal_ok: true,
            leave_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.join_ok && self.route_ok && self.forward_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.heal_ok && self.leave_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.join_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.join_ok {
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
        let c = MeshNet::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MeshNet::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MeshNet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MeshNet::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MeshNet::new();
        c.join_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MeshNet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
