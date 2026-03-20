/// Headlight aim control: auto-leveling, adaptive beam patterns, cornering lights
/// Phase 140

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BeamPattern {
    Low,
    High,
    Adaptive,
    Cornering,
    Fog,
    DRL,
}

impl BeamPattern {
    pub fn lumens(&self) -> u32 {
        match self {
            BeamPattern::Low => 1500,
            BeamPattern::High => 3000,
            BeamPattern::Adaptive => 2500,
            BeamPattern::Cornering => 1200,
            BeamPattern::Fog => 1000,
            BeamPattern::DRL => 600,
        }
    }
    pub fn range_m(&self) -> f64 {
        match self {
            BeamPattern::Low => 60.0,
            BeamPattern::High => 200.0,
            BeamPattern::Adaptive => 150.0,
            BeamPattern::Cornering => 40.0,
            BeamPattern::Fog => 30.0,
            BeamPattern::DRL => 20.0,
        }
    }
    pub fn power_watts(&self) -> f64 {
        match self {
            BeamPattern::Low => 55.0,
            BeamPattern::High => 65.0,
            BeamPattern::Adaptive => 60.0,
            BeamPattern::Cornering => 35.0,
            BeamPattern::Fog => 35.0,
            BeamPattern::DRL => 15.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Headlight {
    pub side: &'static str,
    pub pattern: BeamPattern,
    pub aim_angle_deg: f64,
    pub auto_level: bool,
    pub bulb_life_pct: f64,
}

impl Headlight {
    pub fn new(side: &'static str) -> Self {
        Self {
            side,
            pattern: BeamPattern::Low,
            aim_angle_deg: 0.0,
            auto_level: true,
            bulb_life_pct: 100.0,
        }
    }
    pub fn needs_replacement(&self) -> bool {
        self.bulb_life_pct < 10.0
    }
    pub fn is_misaimed(&self) -> bool {
        self.aim_angle_deg.abs() > 2.0
    }
    pub fn effective_range_m(&self) -> f64 {
        let base = self.pattern.range_m();
        let aim_factor = if self.is_misaimed() { 0.7 } else { 1.0 };
        let life_factor = (self.bulb_life_pct / 100.0).max(0.3);
        base * aim_factor * life_factor
    }
    pub fn recommended_pattern(&self, speed_kmh: f64, ambient_lux: f64) -> BeamPattern {
        if ambient_lux > 10000.0 {
            BeamPattern::DRL
        } else if speed_kmh > 100.0 {
            BeamPattern::High
        } else if speed_kmh > 50.0 {
            BeamPattern::Adaptive
        } else {
            BeamPattern::Low
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeadlightSystem {
    pub left: Headlight,
    pub right: Headlight,
}

impl HeadlightSystem {
    pub fn new() -> Self {
        Self {
            left: Headlight::new("left"),
            right: Headlight::new("right"),
        }
    }
    pub fn total_power_watts(&self) -> f64 {
        self.left.pattern.power_watts() + self.right.pattern.power_watts()
    }
    pub fn min_range_m(&self) -> f64 {
        self.left
            .effective_range_m()
            .min(self.right.effective_range_m())
    }
    pub fn any_needs_replacement(&self) -> bool {
        self.left.needs_replacement() || self.right.needs_replacement()
    }
    pub fn both_aligned(&self) -> bool {
        !self.left.is_misaimed() && !self.right.is_misaimed()
    }
}

impl Default for HeadlightSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lumens() {
        assert_eq!(BeamPattern::High.lumens(), 3000);
    }
    #[test]
    fn test_range() {
        assert!(BeamPattern::High.range_m() > BeamPattern::Low.range_m());
    }
    #[test]
    fn test_needs_replacement() {
        let mut h = Headlight::new("left");
        h.bulb_life_pct = 5.0;
        assert!(h.needs_replacement());
    }
    #[test]
    fn test_misaimed() {
        let mut h = Headlight::new("left");
        h.aim_angle_deg = 3.0;
        assert!(h.is_misaimed());
    }
    #[test]
    fn test_effective_range() {
        let h = Headlight::new("left");
        assert!(h.effective_range_m() > 50.0);
    }
    #[test]
    fn test_recommended_drl() {
        let h = Headlight::new("left");
        assert_eq!(h.recommended_pattern(60.0, 20000.0), BeamPattern::DRL);
    }
    #[test]
    fn test_recommended_high() {
        let h = Headlight::new("left");
        assert_eq!(h.recommended_pattern(120.0, 0.0), BeamPattern::High);
    }
    #[test]
    fn test_system_power() {
        let s = HeadlightSystem::new();
        assert!(s.total_power_watts() > 0.0);
    }
    #[test]
    fn test_both_aligned() {
        let s = HeadlightSystem::new();
        assert!(s.both_aligned());
    }
    #[test]
    fn test_min_range() {
        let s = HeadlightSystem::new();
        assert!(s.min_range_m() > 0.0);
    }
}
