/// Wheel alignment: camber, caster, toe, thrust angle
/// Phase 811

#[derive(Debug, Clone)]
pub struct AlignmentSpec {
    pub camber_ok: bool,
    pub caster_ok: bool,
    pub toe_ok: bool,
    pub thrust_ok: bool,
    pub ride_height_ok: bool,
}

impl Default for AlignmentSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl AlignmentSpec {
    pub fn new() -> Self {
        Self {
            camber_ok: true,
            caster_ok: true,
            toe_ok: true,
            thrust_ok: true,
            ride_height_ok: true,
        }
    }

    pub fn front_ok(&self) -> bool {
        self.camber_ok && self.caster_ok && self.toe_ok
    }

    pub fn geometry_ok(&self) -> bool {
        self.thrust_ok && self.ride_height_ok
    }

    pub fn all_ok(&self) -> bool {
        self.front_ok() && self.geometry_ok()
    }

    pub fn needs_alignment(&self) -> bool {
        !self.camber_ok || !self.toe_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.toe_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_front() {
        let c = AlignmentSpec::new();
        assert!(c.front_ok());
    }

    #[test]
    fn test_geometry() {
        let c = AlignmentSpec::new();
        assert!(c.geometry_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AlignmentSpec::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_align() {
        let c = AlignmentSpec::new();
        assert!(!c.needs_alignment());
    }

    #[test]
    fn test_toe() {
        let mut c = AlignmentSpec::new();
        c.toe_ok = false;
        assert!(c.needs_alignment());
    }

    #[test]
    fn test_health() {
        let c = AlignmentSpec::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
