/// Fuse box: circuit protection, relay slots, connections
/// Phase 521

#[derive(Debug, Clone)]
pub struct FuseBox {
    pub fuse_count: u32,
    pub blown_count: u32,
    pub relay_slots_ok: bool,
    pub connections_ok: bool,
    pub corrosion_free: bool,
}

impl Default for FuseBox {
    fn default() -> Self {
        Self::new()
    }
}

impl FuseBox {
    pub fn new() -> Self {
        Self {
            fuse_count: 40,
            blown_count: 0,
            relay_slots_ok: true,
            connections_ok: true,
            corrosion_free: true,
        }
    }

    pub fn all_fuses_ok(&self) -> bool {
        self.blown_count == 0
    }

    pub fn connections_good(&self) -> bool {
        self.connections_ok && self.corrosion_free
    }

    pub fn all_ok(&self) -> bool {
        self.all_fuses_ok() && self.connections_good() && self.relay_slots_ok
    }

    pub fn needs_service(&self) -> bool {
        self.blown_count > 0 || !self.connections_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.blown_count > 0 { return 30.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuses() {
        let c = FuseBox::new();
        assert!(c.all_fuses_ok());
    }

    #[test]
    fn test_connections() {
        let c = FuseBox::new();
        assert!(c.connections_good());
    }

    #[test]
    fn test_all_ok() {
        let c = FuseBox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FuseBox::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_blown() {
        let mut c = FuseBox::new();
        c.blown_count = 2;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FuseBox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
