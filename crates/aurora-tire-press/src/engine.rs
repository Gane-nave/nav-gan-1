/// Tire pressure monitoring: TPMS sensors, inflation alerts, temperature tracking
/// Phase 168

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TirePosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
    Spare,
}

#[derive(Debug, Clone)]
pub struct TirePressure {
    pub position: TirePosition,
    pub pressure_psi: f64,
    pub target_psi: f64,
    pub temp_c: f64,
}

impl TirePressure {
    pub fn new(position: TirePosition) -> Self {
        Self {
            position,
            pressure_psi: 32.0,
            target_psi: 32.0,
            temp_c: 25.0,
        }
    }

    pub fn deviation_pct(&self) -> f64 {
        ((self.pressure_psi - self.target_psi) / self.target_psi * 100.0).abs()
    }

    pub fn is_low(&self) -> bool {
        self.pressure_psi < self.target_psi * 0.85
    }

    pub fn is_high(&self) -> bool {
        self.pressure_psi > self.target_psi * 1.15
    }

    pub fn is_ok(&self) -> bool {
        !self.is_low() && !self.is_high()
    }

    pub fn overheated(&self) -> bool {
        self.temp_c > 80.0
    }
}

#[derive(Debug, Clone)]
pub struct TpmsSystem {
    pub tires: Vec<TirePressure>,
}

impl Default for TpmsSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TpmsSystem {
    pub fn new() -> Self {
        Self {
            tires: vec![
                TirePressure::new(TirePosition::FrontLeft),
                TirePressure::new(TirePosition::FrontRight),
                TirePressure::new(TirePosition::RearLeft),
                TirePressure::new(TirePosition::RearRight),
            ],
        }
    }

    pub fn all_ok(&self) -> bool {
        self.tires.iter().all(|t| t.is_ok())
    }

    pub fn any_warning(&self) -> bool {
        self.tires.iter().any(|t| t.is_low() || t.is_high())
    }

    pub fn max_deviation_pct(&self) -> f64 {
        self.tires
            .iter()
            .map(|t| t.deviation_pct())
            .fold(0.0_f64, f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let t = TirePressure::new(TirePosition::FrontLeft);
        assert!(t.is_ok());
    }

    #[test]
    fn test_low() {
        let mut t = TirePressure::new(TirePosition::FrontLeft);
        t.pressure_psi = 25.0;
        assert!(t.is_low());
    }

    #[test]
    fn test_high() {
        let mut t = TirePressure::new(TirePosition::RearRight);
        t.pressure_psi = 40.0;
        assert!(t.is_high());
    }

    #[test]
    fn test_deviation() {
        let mut t = TirePressure::new(TirePosition::FrontRight);
        t.pressure_psi = 30.0;
        assert!(t.deviation_pct() > 5.0);
    }

    #[test]
    fn test_system_ok() {
        let s = TpmsSystem::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_system_warning() {
        let mut s = TpmsSystem::new();
        s.tires[0].pressure_psi = 20.0;
        assert!(s.any_warning());
    }

    #[test]
    fn test_overheated() {
        let mut t = TirePressure::new(TirePosition::RearLeft);
        t.temp_c = 90.0;
        assert!(t.overheated());
    }

    #[test]
    fn test_max_deviation() {
        let s = TpmsSystem::new();
        assert!(s.max_deviation_pct() < 1.0);
    }
}
