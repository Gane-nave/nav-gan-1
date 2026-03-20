/// Throttle mapping: pedal response curves, drive modes, acceleration profiles
/// Phase 157

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriveMode {
    Eco,
    Comfort,
    Normal,
    Sport,
    Track,
}

impl DriveMode {
    pub fn throttle_gain(&self) -> f64 {
        match self {
            DriveMode::Eco => 0.6,
            DriveMode::Comfort => 0.8,
            DriveMode::Normal => 1.0,
            DriveMode::Sport => 1.3,
            DriveMode::Track => 1.5,
        }
    }

    pub fn max_power_pct(&self) -> f64 {
        match self {
            DriveMode::Eco => 70.0,
            DriveMode::Comfort => 85.0,
            DriveMode::Normal => 100.0,
            DriveMode::Sport => 100.0,
            DriveMode::Track => 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThrottleMap {
    pub mode: DriveMode,
    pub pedal_position_pct: f64,
    pub current_rpm: f64,
    pub max_rpm: f64,
}

impl Default for ThrottleMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ThrottleMap {
    pub fn new() -> Self {
        Self {
            mode: DriveMode::Normal,
            pedal_position_pct: 0.0,
            current_rpm: 800.0,
            max_rpm: 7000.0,
        }
    }

    pub fn effective_throttle_pct(&self) -> f64 {
        let raw = self.pedal_position_pct * self.mode.throttle_gain();
        raw.min(self.mode.max_power_pct())
    }

    pub fn is_wide_open(&self) -> bool {
        self.pedal_position_pct > 90.0
    }

    pub fn rev_limit_near(&self) -> bool {
        self.current_rpm > self.max_rpm * 0.95
    }

    pub fn should_upshift(&self) -> bool {
        self.current_rpm > self.max_rpm * 0.85
    }

    pub fn power_reserve_pct(&self) -> f64 {
        (self.mode.max_power_pct() - self.effective_throttle_pct()).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throttle_gain() {
        assert!(DriveMode::Sport.throttle_gain() > DriveMode::Eco.throttle_gain());
    }

    #[test]
    fn test_effective_throttle() {
        let mut t = ThrottleMap::new();
        t.pedal_position_pct = 50.0;
        assert!((t.effective_throttle_pct() - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_eco_limits() {
        let mut t = ThrottleMap::new();
        t.mode = DriveMode::Eco;
        t.pedal_position_pct = 100.0;
        assert!(t.effective_throttle_pct() <= 70.0);
    }

    #[test]
    fn test_wide_open() {
        let mut t = ThrottleMap::new();
        t.pedal_position_pct = 95.0;
        assert!(t.is_wide_open());
    }

    #[test]
    fn test_rev_limit() {
        let mut t = ThrottleMap::new();
        t.current_rpm = 6800.0;
        assert!(t.rev_limit_near());
    }

    #[test]
    fn test_upshift() {
        let mut t = ThrottleMap::new();
        t.current_rpm = 6200.0;
        assert!(t.should_upshift());
    }

    #[test]
    fn test_power_reserve() {
        let mut t = ThrottleMap::new();
        t.pedal_position_pct = 30.0;
        assert!(t.power_reserve_pct() > 60.0);
    }

    #[test]
    fn test_max_power() {
        assert!((DriveMode::Normal.max_power_pct() - 100.0).abs() < 0.1);
    }
}
