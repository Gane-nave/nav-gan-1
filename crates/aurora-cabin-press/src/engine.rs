/// Cabin pressure: altitude compensation, seal monitoring, ventilation control
/// Phase 167

#[derive(Debug, Clone)]
pub struct CabinPressureSystem {
    pub cabin_pressure_hpa: f64,
    pub ambient_pressure_hpa: f64,
    pub altitude_m: f64,
    pub seal_integrity_pct: f64,
    pub ventilation_rate_lpm: f64,
    pub pressurized: bool,
}

impl Default for CabinPressureSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CabinPressureSystem {
    pub fn new() -> Self {
        Self {
            cabin_pressure_hpa: 1013.25,
            ambient_pressure_hpa: 1013.25,
            altitude_m: 0.0,
            seal_integrity_pct: 100.0,
            ventilation_rate_lpm: 50.0,
            pressurized: false,
        }
    }

    pub fn pressure_differential_hpa(&self) -> f64 {
        (self.cabin_pressure_hpa - self.ambient_pressure_hpa).abs()
    }

    pub fn is_high_altitude(&self) -> bool {
        self.altitude_m > 2000.0
    }

    pub fn needs_pressurization(&self) -> bool {
        self.altitude_m > 3000.0 && !self.pressurized
    }

    pub fn seal_ok(&self) -> bool {
        self.seal_integrity_pct > 80.0
    }

    pub fn ventilation_adequate(&self) -> bool {
        self.ventilation_rate_lpm >= 30.0
    }

    pub fn ambient_for_altitude(&self) -> f64 {
        1013.25 * (-self.altitude_m / 8500.0_f64).exp()
    }

    pub fn comfort_score(&self) -> f64 {
        let pressure_s = if self.pressure_differential_hpa() < 50.0 {
            40.0
        } else {
            20.0
        };
        let seal_s = self.seal_integrity_pct / 100.0 * 30.0;
        let vent_s = if self.ventilation_adequate() {
            30.0
        } else {
            15.0
        };
        pressure_s + seal_s + vent_s
    }

    pub fn any_warning(&self) -> bool {
        !self.seal_ok() || self.needs_pressurization() || !self.ventilation_adequate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sea_level_pressure() {
        let s = CabinPressureSystem::new();
        assert!((s.pressure_differential_hpa() - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_high_altitude() {
        let mut s = CabinPressureSystem::new();
        s.altitude_m = 3000.0;
        assert!(s.is_high_altitude());
    }

    #[test]
    fn test_not_high() {
        let s = CabinPressureSystem::new();
        assert!(!s.is_high_altitude());
    }

    #[test]
    fn test_needs_pressurization() {
        let mut s = CabinPressureSystem::new();
        s.altitude_m = 4000.0;
        assert!(s.needs_pressurization());
    }

    #[test]
    fn test_seal_ok() {
        let s = CabinPressureSystem::new();
        assert!(s.seal_ok());
    }

    #[test]
    fn test_seal_bad() {
        let mut s = CabinPressureSystem::new();
        s.seal_integrity_pct = 50.0;
        assert!(!s.seal_ok());
    }

    #[test]
    fn test_ventilation() {
        let s = CabinPressureSystem::new();
        assert!(s.ventilation_adequate());
    }

    #[test]
    fn test_comfort_score() {
        let s = CabinPressureSystem::new();
        assert!(s.comfort_score() > 90.0);
    }

    #[test]
    fn test_no_warning() {
        let s = CabinPressureSystem::new();
        assert!(!s.any_warning());
    }

    #[test]
    fn test_ambient_altitude() {
        let mut s = CabinPressureSystem::new();
        s.altitude_m = 1000.0;
        assert!(s.ambient_for_altitude() < 1013.0);
    }
}
