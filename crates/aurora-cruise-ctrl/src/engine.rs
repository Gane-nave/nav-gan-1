/// Cruise control: adaptive speed, following distance, eco mode
/// Phase 150

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CruiseMode {
    Off,
    Standard,
    Adaptive,
    Eco,
}

impl CruiseMode {
    pub fn fuel_efficiency_factor(&self) -> f64 {
        match self {
            CruiseMode::Off => 1.0,
            CruiseMode::Standard => 1.05,
            CruiseMode::Adaptive => 1.08,
            CruiseMode::Eco => 1.15,
        }
    }
    pub fn is_active(&self) -> bool {
        !matches!(self, CruiseMode::Off)
    }
}

#[derive(Debug, Clone)]
pub struct CruiseSystem {
    pub mode: CruiseMode,
    pub set_speed_kmh: f64,
    pub current_speed_kmh: f64,
    pub following_distance_m: f64,
    pub min_following_distance_m: f64,
}

impl Default for CruiseSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CruiseSystem {
    pub fn new() -> Self {
        Self {
            mode: CruiseMode::Off,
            set_speed_kmh: 0.0,
            current_speed_kmh: 0.0,
            following_distance_m: 50.0,
            min_following_distance_m: 20.0,
        }
    }
    pub fn speed_error(&self) -> f64 {
        self.current_speed_kmh - self.set_speed_kmh
    }
    pub fn needs_accel(&self) -> bool {
        self.mode.is_active() && self.speed_error() < -2.0
    }
    pub fn needs_decel(&self) -> bool {
        self.mode.is_active() && (self.speed_error() > 2.0 || self.too_close())
    }
    pub fn too_close(&self) -> bool {
        self.following_distance_m < self.min_following_distance_m
    }
    pub fn following_time_sec(&self) -> f64 {
        if self.current_speed_kmh > 0.0 {
            self.following_distance_m / (self.current_speed_kmh / 3.6)
        } else {
            f64::MAX
        }
    }
    pub fn safe_following(&self) -> bool {
        self.following_time_sec() >= 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_active() {
        assert!(CruiseMode::Adaptive.is_active());
        assert!(!CruiseMode::Off.is_active());
    }

    #[test]
    fn test_fuel_efficiency() {
        assert!(
            CruiseMode::Eco.fuel_efficiency_factor()
                > CruiseMode::Standard.fuel_efficiency_factor()
        );
    }

    #[test]
    fn test_needs_accel() {
        let mut s = CruiseSystem::new();
        s.mode = CruiseMode::Standard;
        s.set_speed_kmh = 100.0;
        s.current_speed_kmh = 90.0;
        assert!(s.needs_accel());
    }

    #[test]
    fn test_needs_decel() {
        let mut s = CruiseSystem::new();
        s.mode = CruiseMode::Standard;
        s.set_speed_kmh = 100.0;
        s.current_speed_kmh = 110.0;
        assert!(s.needs_decel());
    }

    #[test]
    fn test_too_close() {
        let mut s = CruiseSystem::new();
        s.following_distance_m = 10.0;
        assert!(s.too_close());
    }

    #[test]
    fn test_safe_following() {
        let mut s = CruiseSystem::new();
        s.current_speed_kmh = 50.0;
        s.following_distance_m = 50.0;
        assert!(s.safe_following());
    }

    #[test]
    fn test_following_time() {
        let mut s = CruiseSystem::new();
        s.current_speed_kmh = 36.0;
        s.following_distance_m = 20.0;
        assert!((s.following_time_sec() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_speed_error() {
        let mut s = CruiseSystem::new();
        s.set_speed_kmh = 100.0;
        s.current_speed_kmh = 95.0;
        assert!((s.speed_error() - (-5.0)).abs() < 0.1);
    }
}
