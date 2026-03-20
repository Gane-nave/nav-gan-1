/// Seat heating/cooling: temperature control, occupancy detection, zones
/// Phase 178

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeatLevel {
    Off,
    Low,
    Medium,
    High,
}

impl HeatLevel {
    pub fn power_watts(&self) -> f64 {
        match self {
            HeatLevel::Off => 0.0,
            HeatLevel::Low => 30.0,
            HeatLevel::Medium => 60.0,
            HeatLevel::High => 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SeatHeater {
    pub level: HeatLevel,
    pub current_temp_c: f64,
    pub target_temp_c: f64,
    pub occupied: bool,
    pub cooling_available: bool,
}

impl Default for SeatHeater {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatHeater {
    pub fn new() -> Self {
        Self {
            level: HeatLevel::Off,
            current_temp_c: 22.0,
            target_temp_c: 30.0,
            occupied: true,
            cooling_available: false,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.current_temp_c - self.target_temp_c).abs() < 2.0
    }

    pub fn needs_heating(&self) -> bool {
        self.occupied && self.current_temp_c < self.target_temp_c - 2.0
    }

    pub fn power_draw_w(&self) -> f64 {
        if self.occupied {
            self.level.power_watts()
        } else {
            0.0
        }
    }

    pub fn comfort_score(&self) -> f64 {
        if !self.occupied {
            return 100.0;
        }
        let temp_diff = (self.current_temp_c - self.target_temp_c).abs();
        (100.0 - temp_diff * 10.0).clamp(0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_off() {
        assert!((HeatLevel::Off.power_watts() - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_power_high() {
        assert!((HeatLevel::High.power_watts() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_needs_heating() {
        let s = SeatHeater::new();
        assert!(s.needs_heating());
    }

    #[test]
    fn test_at_target() {
        let mut s = SeatHeater::new();
        s.current_temp_c = 30.0;
        assert!(s.at_target());
    }

    #[test]
    fn test_power_draw() {
        let mut s = SeatHeater::new();
        s.level = HeatLevel::High;
        assert!((s.power_draw_w() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_no_power_unoccupied() {
        let mut s = SeatHeater::new();
        s.level = HeatLevel::High;
        s.occupied = false;
        assert!((s.power_draw_w() - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_comfort() {
        let mut s = SeatHeater::new();
        s.current_temp_c = 30.0;
        assert!(s.comfort_score() > 90.0);
    }
}
