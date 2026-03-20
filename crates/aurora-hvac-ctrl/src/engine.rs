/// HVAC control: climate zones, temperature regulation, fan speed
/// Phase 234

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HvacMode {
    Auto,
    Heat,
    Cool,
    Defrost,
    Off,
}

#[derive(Debug, Clone)]
pub struct HvacController {
    pub mode: HvacMode,
    pub target_temp_c: f64,
    pub cabin_temp_c: f64,
    pub fan_speed: u8,
    pub recirculate: bool,
    pub dual_zone: bool,
}

impl Default for HvacController {
    fn default() -> Self {
        Self::new()
    }
}

impl HvacController {
    pub fn new() -> Self {
        Self {
            mode: HvacMode::Auto,
            target_temp_c: 22.0,
            cabin_temp_c: 22.0,
            fan_speed: 3,
            recirculate: false,
            dual_zone: true,
        }
    }

    pub fn temp_error_c(&self) -> f64 {
        (self.cabin_temp_c - self.target_temp_c).abs()
    }

    pub fn at_target(&self) -> bool {
        self.temp_error_c() < 1.0
    }

    pub fn needs_cooling(&self) -> bool {
        self.cabin_temp_c > self.target_temp_c + 2.0
    }

    pub fn needs_heating(&self) -> bool {
        self.cabin_temp_c < self.target_temp_c - 2.0
    }

    pub fn power_draw_w(&self) -> f64 {
        let base = self.fan_speed as f64 * 50.0;
        match self.mode {
            HvacMode::Cool => base + 500.0,
            HvacMode::Heat => base + 200.0,
            HvacMode::Defrost => base + 300.0,
            HvacMode::Auto => base + 100.0,
            HvacMode::Off => 0.0,
        }
    }

    pub fn health_score(&self) -> f64 {
        if self.mode == HvacMode::Off {
            return 50.0;
        }
        if self.at_target() {
            100.0
        } else {
            70.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_target() {
        let h = HvacController::new();
        assert!(h.at_target());
    }

    #[test]
    fn test_no_cooling() {
        let h = HvacController::new();
        assert!(!h.needs_cooling());
    }

    #[test]
    fn test_no_heating() {
        let h = HvacController::new();
        assert!(!h.needs_heating());
    }

    #[test]
    fn test_power_draw() {
        let h = HvacController::new();
        assert!(h.power_draw_w() > 200.0);
    }

    #[test]
    fn test_hot_cabin() {
        let mut h = HvacController::new();
        h.cabin_temp_c = 30.0;
        assert!(h.needs_cooling());
    }

    #[test]
    fn test_health() {
        let h = HvacController::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
