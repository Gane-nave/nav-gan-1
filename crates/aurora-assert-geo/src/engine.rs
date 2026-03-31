/// aurora-assert-geo: assert geo
/// Phase 2502

#[derive(Debug, Clone)]
pub struct AssertGeo {
    pub position_ok: bool,
    pub distance_ok: bool,
    pub area_ok: bool,
    pub route_ok: bool,
    pub bearing_ok: bool,
}

impl Default for AssertGeo {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertGeo {
    pub fn new() -> Self {
        Self {
            position_ok: true,
            distance_ok: true,
            area_ok: true,
            route_ok: true,
            bearing_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.position_ok && self.distance_ok && self.area_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.route_ok && self.bearing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.position_ok || !self.distance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.position_ok {
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
        let c = AssertGeo::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertGeo::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertGeo::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertGeo::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertGeo::new();
        c.position_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertGeo::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertGeo::default();
        assert!(c.all_ok());
    }
}
