/// Brake assist: emergency braking, brake fade detection, pedal force amplification
/// Phase 154

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrakeMode {
    Normal,
    Emergency,
    Fade,
    Regenerative,
}

#[derive(Debug, Clone)]
pub struct BrakeSystem {
    pub mode: BrakeMode,
    pub pedal_force_n: f64,
    pub brake_temp_c: f64,
    pub pad_thickness_mm: f64,
    pub fluid_level_pct: f64,
    pub decel_g: f64,
}

impl Default for BrakeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeSystem {
    pub fn new() -> Self {
        Self {
            mode: BrakeMode::Normal,
            pedal_force_n: 0.0,
            brake_temp_c: 25.0,
            pad_thickness_mm: 12.0,
            fluid_level_pct: 100.0,
            decel_g: 0.0,
        }
    }

    pub fn is_emergency(&self) -> bool {
        self.pedal_force_n > 200.0 || self.decel_g > 0.8
    }

    pub fn brake_fade_risk(&self) -> f64 {
        if self.brake_temp_c > 400.0 {
            0.9
        } else if self.brake_temp_c > 300.0 {
            0.5
        } else if self.brake_temp_c > 200.0 {
            0.2
        } else {
            0.0
        }
    }

    pub fn pads_need_replacement(&self) -> bool {
        self.pad_thickness_mm < 3.0
    }

    pub fn fluid_low(&self) -> bool {
        self.fluid_level_pct < 20.0
    }

    pub fn stopping_distance_m(&self, speed_kmh: f64) -> f64 {
        let speed_ms = speed_kmh / 3.6;
        let decel = if self.decel_g > 0.0 {
            self.decel_g * 9.81
        } else {
            8.0
        };
        (speed_ms * speed_ms) / (2.0 * decel)
    }

    pub fn brake_health_score(&self) -> f64 {
        let pad_score = (self.pad_thickness_mm / 12.0).min(1.0) * 40.0;
        let fluid_score = (self.fluid_level_pct / 100.0) * 30.0;
        let temp_score = (1.0 - self.brake_fade_risk()) * 30.0;
        pad_score + fluid_score + temp_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergency() {
        let mut s = BrakeSystem::new();
        s.pedal_force_n = 250.0;
        assert!(s.is_emergency());
    }

    #[test]
    fn test_not_emergency() {
        let s = BrakeSystem::new();
        assert!(!s.is_emergency());
    }

    #[test]
    fn test_fade_risk_cool() {
        let s = BrakeSystem::new();
        assert!((s.brake_fade_risk() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_fade_risk_hot() {
        let mut s = BrakeSystem::new();
        s.brake_temp_c = 450.0;
        assert!(s.brake_fade_risk() > 0.8);
    }

    #[test]
    fn test_pads_ok() {
        let s = BrakeSystem::new();
        assert!(!s.pads_need_replacement());
    }

    #[test]
    fn test_pads_worn() {
        let mut s = BrakeSystem::new();
        s.pad_thickness_mm = 2.0;
        assert!(s.pads_need_replacement());
    }

    #[test]
    fn test_stopping_distance() {
        let s = BrakeSystem::new();
        let dist = s.stopping_distance_m(100.0);
        assert!(dist > 30.0 && dist < 60.0);
    }

    #[test]
    fn test_health_score() {
        let s = BrakeSystem::new();
        assert!(s.brake_health_score() > 90.0);
    }
}
