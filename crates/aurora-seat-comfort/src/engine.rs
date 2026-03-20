/// Seat comfort: position memory, lumbar support, heating/cooling, massage
/// Phase 137

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeatZone {
    Driver,
    Passenger,
    RearLeft,
    RearRight,
    RearCenter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeatFeature {
    Heating,
    Cooling,
    Lumbar,
    Massage,
    Ventilation,
}

impl SeatFeature {
    pub fn power_watts(&self) -> f64 {
        match self {
            SeatFeature::Heating => 80.0,
            SeatFeature::Cooling => 60.0,
            SeatFeature::Lumbar => 10.0,
            SeatFeature::Massage => 30.0,
            SeatFeature::Ventilation => 40.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SeatProfile {
    pub zone: SeatZone,
    pub recline_deg: f64,
    pub height_mm: f64,
    pub lumbar_level: u8,
    pub heating_level: u8,
    pub cooling_level: u8,
    pub massage_on: bool,
}

impl SeatProfile {
    pub fn new(zone: SeatZone) -> Self {
        Self {
            zone,
            recline_deg: 110.0,
            height_mm: 500.0,
            lumbar_level: 3,
            heating_level: 0,
            cooling_level: 0,
            massage_on: false,
        }
    }
    pub fn comfort_score(&self) -> f64 {
        let recline_score = if (100.0..=120.0).contains(&self.recline_deg) {
            90.0
        } else {
            60.0
        };
        let lumbar_score = (self.lumbar_level as f64 / 5.0) * 100.0;
        (recline_score * 0.6 + lumbar_score * 0.4).min(100.0)
    }
    pub fn is_ergonomic(&self) -> bool {
        self.comfort_score() > 70.0
    }
    pub fn active_features(&self) -> Vec<SeatFeature> {
        let mut f = Vec::new();
        if self.heating_level > 0 {
            f.push(SeatFeature::Heating);
        }
        if self.cooling_level > 0 {
            f.push(SeatFeature::Cooling);
        }
        if self.lumbar_level > 0 {
            f.push(SeatFeature::Lumbar);
        }
        if self.massage_on {
            f.push(SeatFeature::Massage);
        }
        f
    }
    pub fn total_power_watts(&self) -> f64 {
        self.active_features().iter().map(|f| f.power_watts()).sum()
    }
    pub fn fatigue_reduction_pct(&self) -> f64 {
        let mut reduction: f64 = 0.0;
        if self.lumbar_level >= 3 {
            reduction += 20.0;
        }
        if self.massage_on {
            reduction += 15.0;
        }
        if (105.0..=115.0).contains(&self.recline_deg) {
            reduction += 10.0;
        }
        reduction.min(50.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_feature_power() {
        assert_eq!(SeatFeature::Heating.power_watts(), 80.0);
    }
    #[test]
    fn test_comfort_score() {
        let s = SeatProfile::new(SeatZone::Driver);
        assert!(s.comfort_score() > 50.0);
    }
    #[test]
    fn test_ergonomic() {
        let s = SeatProfile::new(SeatZone::Driver);
        assert!(s.is_ergonomic());
    }
    #[test]
    fn test_active_features() {
        let mut s = SeatProfile::new(SeatZone::Driver);
        s.heating_level = 2;
        s.massage_on = true;
        assert_eq!(s.active_features().len(), 3); // heating + lumbar + massage
    }
    #[test]
    fn test_total_power() {
        let mut s = SeatProfile::new(SeatZone::Driver);
        s.heating_level = 2;
        assert!(s.total_power_watts() > 0.0);
    }
    #[test]
    fn test_fatigue_reduction() {
        let mut s = SeatProfile::new(SeatZone::Driver);
        s.massage_on = true;
        s.recline_deg = 110.0;
        assert!(s.fatigue_reduction_pct() > 20.0);
    }
    #[test]
    fn test_no_cooling() {
        let s = SeatProfile::new(SeatZone::Passenger);
        assert_eq!(s.cooling_level, 0);
    }
    #[test]
    fn test_default_recline() {
        let s = SeatProfile::new(SeatZone::Driver);
        assert!((s.recline_deg - 110.0).abs() < 0.1);
    }
}
