/// Sunroof control: open/close/tilt, rain sensing, wind deflector
/// Phase 176

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SunroofState {
    Closed,
    Tilted,
    PartialOpen,
    FullyOpen,
}

#[derive(Debug, Clone)]
pub struct SunroofSystem {
    pub state: SunroofState,
    pub open_pct: f64,
    pub wind_deflector_up: bool,
    pub rain_close_enabled: bool,
    pub speed_kmh: f64,
}

impl Default for SunroofSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SunroofSystem {
    pub fn new() -> Self {
        Self {
            state: SunroofState::Closed,
            open_pct: 0.0,
            wind_deflector_up: false,
            rain_close_enabled: true,
            speed_kmh: 0.0,
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.state, SunroofState::Closed)
    }

    pub fn needs_deflector(&self) -> bool {
        !self.is_closed() && self.speed_kmh > 40.0
    }

    pub fn wind_noise_risk(&self) -> bool {
        self.open_pct > 30.0 && self.speed_kmh > 80.0
    }

    pub fn should_auto_close_rain(&self) -> bool {
        self.rain_close_enabled && !self.is_closed()
    }

    pub fn drag_coefficient_increase(&self) -> f64 {
        self.open_pct / 100.0 * 0.03
    }

    pub fn cabin_temp_effect_c(&self) -> f64 {
        if self.is_closed() {
            0.0
        } else {
            self.open_pct / 100.0 * 5.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closed() {
        let s = SunroofSystem::new();
        assert!(s.is_closed());
    }

    #[test]
    fn test_deflector_needed() {
        let mut s = SunroofSystem::new();
        s.state = SunroofState::FullyOpen;
        s.speed_kmh = 60.0;
        assert!(s.needs_deflector());
    }

    #[test]
    fn test_wind_noise() {
        let mut s = SunroofSystem::new();
        s.open_pct = 50.0;
        s.speed_kmh = 100.0;
        assert!(s.wind_noise_risk());
    }

    #[test]
    fn test_auto_close() {
        let mut s = SunroofSystem::new();
        s.state = SunroofState::Tilted;
        assert!(s.should_auto_close_rain());
    }

    #[test]
    fn test_drag() {
        let mut s = SunroofSystem::new();
        s.open_pct = 100.0;
        assert!((s.drag_coefficient_increase() - 0.03).abs() < 0.001);
    }

    #[test]
    fn test_no_drag_closed() {
        let s = SunroofSystem::new();
        assert!((s.drag_coefficient_increase() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_temp_effect() {
        let mut s = SunroofSystem::new();
        s.state = SunroofState::FullyOpen;
        s.open_pct = 100.0;
        assert!((s.cabin_temp_effect_c() - 5.0).abs() < 0.1);
    }
}
