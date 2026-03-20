/// Subframe: bushing, mount, crossmember, rust
/// Phase 648

#[derive(Debug, Clone)]
pub struct Subframe {
    pub bushing_ok: bool,
    pub mount_ok: bool,
    pub crossmember_ok: bool,
    pub rust_free: bool,
    pub bolts_ok: bool,
}

impl Default for Subframe {
    fn default() -> Self {
        Self::new()
    }
}

impl Subframe {
    pub fn new() -> Self {
        Self {
            bushing_ok: true,
            mount_ok: true,
            crossmember_ok: true,
            rust_free: true,
            bolts_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.crossmember_ok && self.rust_free
    }

    pub fn mounting_ok(&self) -> bool {
        self.bushing_ok && self.mount_ok && self.bolts_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.mounting_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.rust_free || !self.bushing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rust_free { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = Subframe::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_mounting() {
        let c = Subframe::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Subframe::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Subframe::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_rust() {
        let mut c = Subframe::new();
        c.rust_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Subframe::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
