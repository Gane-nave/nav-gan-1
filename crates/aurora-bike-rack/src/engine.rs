/// Bike rack: hitch mount, arm, strap, lock
/// Phase 849

#[derive(Debug, Clone)]
pub struct BikeRack {
    pub hitch_ok: bool,
    pub arm_ok: bool,
    pub strap_ok: bool,
    pub lock_ok: bool,
    pub tilt_ok: bool,
}

impl Default for BikeRack {
    fn default() -> Self {
        Self::new()
    }
}

impl BikeRack {
    pub fn new() -> Self {
        Self {
            hitch_ok: true,
            arm_ok: true,
            strap_ok: true,
            lock_ok: true,
            tilt_ok: true,
        }
    }

    pub fn mounting_ok(&self) -> bool {
        self.hitch_ok && self.arm_ok
    }

    pub fn securing_ok(&self) -> bool {
        self.strap_ok && self.lock_ok && self.tilt_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mounting_ok() && self.securing_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.hitch_ok || !self.strap_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hitch_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mounting() {
        let c = BikeRack::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_securing() {
        let c = BikeRack::new();
        assert!(c.securing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BikeRack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BikeRack::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_hitch() {
        let mut c = BikeRack::new();
        c.hitch_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BikeRack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
