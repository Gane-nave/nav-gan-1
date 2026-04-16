/// Anti-lock braking system: wheel speed monitoring, brake pulse control
/// Phase 151

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WheelPosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

#[derive(Debug, Clone)]
pub struct WheelSensor {
    pub position: WheelPosition,
    pub speed_rpm: f64,
    pub brake_pressure_psi: f64,
    pub slip_ratio: f64,
}

impl WheelSensor {
    pub fn new(position: WheelPosition) -> Self {
        Self {
            position,
            speed_rpm: 0.0,
            brake_pressure_psi: 0.0,
            slip_ratio: 0.0,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.slip_ratio > 0.3
    }

    pub fn needs_abs(&self) -> bool {
        self.slip_ratio > 0.15 && self.brake_pressure_psi > 10.0
    }

    pub fn speed_kmh(&self) -> f64 {
        self.speed_rpm * 0.1885
    }
}

#[derive(Debug, Clone)]
pub struct AbsSystem {
    pub enabled: bool,
    pub wheels: Vec<WheelSensor>,
    pub interventions: u64,
}

impl Default for AbsSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AbsSystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            wheels: vec![
                WheelSensor::new(WheelPosition::FrontLeft),
                WheelSensor::new(WheelPosition::FrontRight),
                WheelSensor::new(WheelPosition::RearLeft),
                WheelSensor::new(WheelPosition::RearRight),
            ],
            interventions: 0,
        }
    }

    pub fn any_locked(&self) -> bool {
        self.wheels.iter().any(|w| w.is_locked())
    }

    pub fn needs_intervention(&self) -> bool {
        self.enabled && self.wheels.iter().any(|w| w.needs_abs())
    }

    pub fn max_slip(&self) -> f64 {
        self.wheels
            .iter()
            .map(|w| w.slip_ratio)
            .fold(0.0_f64, f64::max)
    }

    pub fn average_speed_kmh(&self) -> f64 {
        if self.wheels.is_empty() {
            return 0.0;
        }
        let total: f64 = self.wheels.iter().map(|w| w.speed_kmh()).sum();
        total / self.wheels.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wheel_locked() {
        let mut w = WheelSensor::new(WheelPosition::FrontLeft);
        w.slip_ratio = 0.5;
        assert!(w.is_locked());
    }

    #[test]
    fn test_wheel_not_locked() {
        let w = WheelSensor::new(WheelPosition::FrontRight);
        assert!(!w.is_locked());
    }

    #[test]
    fn test_needs_abs() {
        let mut w = WheelSensor::new(WheelPosition::RearLeft);
        w.slip_ratio = 0.2;
        w.brake_pressure_psi = 50.0;
        assert!(w.needs_abs());
    }

    #[test]
    fn test_speed_kmh() {
        let mut w = WheelSensor::new(WheelPosition::FrontLeft);
        w.speed_rpm = 500.0;
        assert!(w.speed_kmh() > 90.0);
    }

    #[test]
    fn test_system_no_lock() {
        let s = AbsSystem::new();
        assert!(!s.any_locked());
    }

    #[test]
    fn test_system_intervention() {
        let mut s = AbsSystem::new();
        s.wheels[0].slip_ratio = 0.2;
        s.wheels[0].brake_pressure_psi = 50.0;
        assert!(s.needs_intervention());
    }

    #[test]
    fn test_max_slip() {
        let mut s = AbsSystem::new();
        s.wheels[1].slip_ratio = 0.25;
        assert!((s.max_slip() - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_avg_speed() {
        let s = AbsSystem::new();
        assert!((s.average_speed_kmh() - 0.0).abs() < 0.01);
    }
}
