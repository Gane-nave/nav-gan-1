/// tja sys: engage, follow, steer, brake, disengage
/// Phase 1168

#[derive(Debug, Clone)]
pub struct TjaSys {
    pub engage_ok: bool,
    pub follow_ok: bool,
    pub steer_ok: bool,
    pub brake_ok: bool,
    pub disengage_ok: bool,
}

impl Default for TjaSys {
    fn default() -> Self {
        Self::new()
    }
}

impl TjaSys {
    pub fn new() -> Self {
        Self {
            engage_ok: true,
            follow_ok: true,
            steer_ok: true,
            brake_ok: true,
            disengage_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.engage_ok && self.follow_ok && self.steer_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.brake_ok && self.disengage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.engage_ok || !self.follow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.engage_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TjaSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TjaSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TjaSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TjaSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TjaSys::new();
        c.engage_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TjaSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
