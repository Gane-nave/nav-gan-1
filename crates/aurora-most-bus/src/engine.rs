/// MOST bus: optical fiber, ring, synchronous, async
/// Phase 731

#[derive(Debug, Clone)]
pub struct MostBus {
    pub fiber_ok: bool,
    pub ring_ok: bool,
    pub sync_ok: bool,
    pub async_ok: bool,
    pub bandwidth_ok: bool,
}

impl Default for MostBus {
    fn default() -> Self {
        Self::new()
    }
}

impl MostBus {
    pub fn new() -> Self {
        Self {
            fiber_ok: true,
            ring_ok: true,
            sync_ok: true,
            async_ok: true,
            bandwidth_ok: true,
        }
    }

    pub fn physical_ok(&self) -> bool {
        self.fiber_ok && self.ring_ok
    }

    pub fn transport_ok(&self) -> bool {
        self.sync_ok && self.async_ok && self.bandwidth_ok
    }

    pub fn all_ok(&self) -> bool {
        self.physical_ok() && self.transport_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.fiber_ok || !self.ring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fiber_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical() {
        let c = MostBus::new();
        assert!(c.physical_ok());
    }

    #[test]
    fn test_transport() {
        let c = MostBus::new();
        assert!(c.transport_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MostBus::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = MostBus::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_fiber() {
        let mut c = MostBus::new();
        c.fiber_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = MostBus::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
