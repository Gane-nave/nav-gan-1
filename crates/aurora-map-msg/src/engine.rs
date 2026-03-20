/// map message: lane, intersection, geometry, approach, egress
/// Phase 1130

#[derive(Debug, Clone)]
pub struct MapMsg {
    pub lane_ok: bool,
    pub intersection_ok: bool,
    pub geometry_ok: bool,
    pub approach_ok: bool,
    pub egress_ok: bool,
}

impl Default for MapMsg {
    fn default() -> Self {
        Self::new()
    }
}

impl MapMsg {
    pub fn new() -> Self {
        Self {
            lane_ok: true,
            intersection_ok: true,
            geometry_ok: true,
            approach_ok: true,
            egress_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.lane_ok && self.intersection_ok && self.geometry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.approach_ok && self.egress_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.lane_ok || !self.intersection_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lane_ok {
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
        let c = MapMsg::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapMsg::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapMsg::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapMsg::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapMsg::new();
        c.lane_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapMsg::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
