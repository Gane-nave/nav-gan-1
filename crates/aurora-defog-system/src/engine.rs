/// Defog/demist system: humidity sensing, auto-defog, zone control
/// Phase 136

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefogZone {
    Windshield,
    RearWindow,
    SideMirrors,
    AllZones,
}

impl DefogZone {
    pub fn power_watts(&self) -> f64 {
        match self {
            DefogZone::Windshield => 500.0,
            DefogZone::RearWindow => 300.0,
            DefogZone::SideMirrors => 100.0,
            DefogZone::AllZones => 900.0,
        }
    }
    pub fn defog_time_sec(&self, humidity_pct: f64) -> f64 {
        let base = match self {
            DefogZone::Windshield => 120.0,
            DefogZone::RearWindow => 90.0,
            DefogZone::SideMirrors => 60.0,
            DefogZone::AllZones => 150.0,
        };
        base * (humidity_pct / 50.0).max(1.0)
    }
}

#[derive(Debug, Clone)]
pub struct DefogSystem {
    pub interior_humidity_pct: f64,
    pub exterior_temp_c: f64,
    pub interior_temp_c: f64,
    pub active_zones: Vec<DefogZone>,
    pub ac_available: bool,
}

impl DefogSystem {
    pub fn new(humidity: f64, ext_temp: f64, int_temp: f64) -> Self {
        Self {
            interior_humidity_pct: humidity,
            exterior_temp_c: ext_temp,
            interior_temp_c: int_temp,
            active_zones: Vec::new(),
            ac_available: true,
        }
    }
    pub fn fog_risk(&self) -> f64 {
        let dew_point_diff = self.interior_temp_c - self.exterior_temp_c;
        let humidity_factor = self.interior_humidity_pct / 100.0;
        let risk: f64 = (dew_point_diff * 5.0 + humidity_factor * 50.0).min(100.0);
        risk.max(0.0)
    }
    pub fn needs_defog(&self) -> bool {
        self.fog_risk() > 40.0
    }
    pub fn recommended_zones(&self) -> Vec<DefogZone> {
        if self.fog_risk() > 70.0 {
            vec![DefogZone::AllZones]
        } else if self.fog_risk() > 40.0 {
            vec![DefogZone::Windshield]
        } else {
            vec![]
        }
    }
    pub fn total_power_draw_watts(&self) -> f64 {
        self.active_zones.iter().map(|z| z.power_watts()).sum()
    }
    pub fn estimated_clear_time_sec(&self) -> f64 {
        self.active_zones
            .iter()
            .map(|z| z.defog_time_sec(self.interior_humidity_pct))
            .fold(0.0_f64, f64::max)
    }
    pub fn ac_recommended(&self) -> bool {
        self.interior_humidity_pct > 60.0 && self.ac_available
    }
    pub fn visibility_impact_pct(&self) -> f64 {
        if self.fog_risk() > 70.0 {
            60.0
        } else if self.fog_risk() > 40.0 {
            30.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zone_power() {
        assert_eq!(DefogZone::Windshield.power_watts(), 500.0);
    }
    #[test]
    fn test_defog_time() {
        assert!(DefogZone::Windshield.defog_time_sec(80.0) > 120.0);
    }
    #[test]
    fn test_fog_risk_high() {
        let s = DefogSystem::new(90.0, 5.0, 22.0);
        assert!(s.fog_risk() > 50.0);
    }
    #[test]
    fn test_fog_risk_low() {
        let s = DefogSystem::new(30.0, 20.0, 22.0);
        assert!(s.fog_risk() < 40.0);
    }
    #[test]
    fn test_needs_defog() {
        let s = DefogSystem::new(90.0, 5.0, 22.0);
        assert!(s.needs_defog());
    }
    #[test]
    fn test_no_defog() {
        let s = DefogSystem::new(30.0, 20.0, 22.0);
        assert!(!s.needs_defog());
    }
    #[test]
    fn test_recommended_zones() {
        let s = DefogSystem::new(90.0, 0.0, 22.0);
        assert!(!s.recommended_zones().is_empty());
    }
    #[test]
    fn test_power_draw() {
        let mut s = DefogSystem::new(80.0, 5.0, 22.0);
        s.active_zones.push(DefogZone::Windshield);
        s.active_zones.push(DefogZone::RearWindow);
        assert!((s.total_power_draw_watts() - 800.0).abs() < 0.1);
    }
    #[test]
    fn test_ac_recommended() {
        let s = DefogSystem::new(80.0, 10.0, 22.0);
        assert!(s.ac_recommended());
    }
    #[test]
    fn test_visibility_impact() {
        let s = DefogSystem::new(90.0, 0.0, 22.0);
        assert!(s.visibility_impact_pct() > 0.0);
    }
}
