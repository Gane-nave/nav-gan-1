/// Fan clutch: engagement, viscous coupling, temp control
/// Phase 515

#[derive(Debug, Clone)]
pub struct FanClutch {
    pub engagement_pct: f64,
    pub coolant_temp_c: f64,
    pub engage_temp_c: f64,
    pub bearing_ok: bool,
    pub fluid_ok: bool,
}

impl Default for FanClutch {
    fn default() -> Self {
        Self::new()
    }
}

impl FanClutch {
    pub fn new() -> Self {
        Self {
            engagement_pct: 40.0,
            coolant_temp_c: 90.0,
            engage_temp_c: 95.0,
            bearing_ok: true,
            fluid_ok: true,
        }
    }

    pub fn should_engage(&self) -> bool {
        self.coolant_temp_c > self.engage_temp_c
    }

    pub fn is_engaged(&self) -> bool {
        self.engagement_pct > 70.0
    }

    pub fn all_ok(&self) -> bool {
        self.bearing_ok && self.fluid_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bearing_ok || !self.fluid_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_engage() {
        let c = FanClutch::new();
        assert!(!c.should_engage());
    }

    #[test]
    fn test_not_engaged() {
        let c = FanClutch::new();
        assert!(!c.is_engaged());
    }

    #[test]
    fn test_all_ok() {
        let c = FanClutch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = FanClutch::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bearing() {
        let mut c = FanClutch::new();
        c.bearing_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = FanClutch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
