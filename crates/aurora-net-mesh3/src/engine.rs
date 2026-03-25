/// net mesh3: discover, connect, route, heal, log
/// Phase 2265

#[derive(Debug, Clone)]
pub struct NetMesh3 {
    pub discover_ok: bool,
    pub connect_ok: bool,
    pub route_ok: bool,
    pub heal_ok: bool,
    pub log_ok: bool,
}

impl Default for NetMesh3 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetMesh3 {
    pub fn new() -> Self {
        Self {
            discover_ok: true,
            connect_ok: true,
            route_ok: true,
            heal_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.discover_ok && self.connect_ok && self.route_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.heal_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.discover_ok || !self.connect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.discover_ok {
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
        let c = NetMesh3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetMesh3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetMesh3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetMesh3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetMesh3::new();
        c.discover_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetMesh3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
