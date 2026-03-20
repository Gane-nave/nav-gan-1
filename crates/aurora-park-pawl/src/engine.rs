/// Park pawl: parking lock, engagement mechanism, safety interlock
/// Phase 468

#[derive(Debug, Clone)]
pub struct ParkPawl {
    pub engaged: bool,
    pub spring_ok: bool,
    pub teeth_ok: bool,
    pub actuator_ok: bool,
    pub wear_pct: f64,
}

impl Default for ParkPawl {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkPawl {
    pub fn new() -> Self {
        Self {
            engaged: true,
            spring_ok: true,
            teeth_ok: true,
            actuator_ok: true,
            wear_pct: 10.0,
        }
    }

    pub fn locked(&self) -> bool {
        self.engaged && self.teeth_ok
    }

    pub fn all_ok(&self) -> bool {
        self.spring_ok && self.teeth_ok && self.actuator_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.teeth_ok && self.spring_ok
    }

    pub fn needs_service(&self) -> bool {
        self.wear_pct > 70.0 || !self.teeth_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.teeth_ok {
            return 0.0;
        }
        if !self.spring_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locked() {
        let p = ParkPawl::new();
        assert!(p.locked());
    }

    #[test]
    fn test_all_ok() {
        let p = ParkPawl::new();
        assert!(p.all_ok());
    }

    #[test]
    fn test_safety() {
        let p = ParkPawl::new();
        assert!(p.safety_ok());
    }

    #[test]
    fn test_no_service() {
        let p = ParkPawl::new();
        assert!(!p.needs_service());
    }

    #[test]
    fn test_worn_teeth() {
        let mut p = ParkPawl::new();
        p.teeth_ok = false;
        assert!(p.needs_service());
    }

    #[test]
    fn test_health() {
        let p = ParkPawl::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
