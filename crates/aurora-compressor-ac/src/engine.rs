/// AC compressor: refrigerant pressure, clutch engagement, cooling capacity
/// Phase 235

#[derive(Debug, Clone)]
pub struct AcCompressor {
    pub high_pressure_bar: f64,
    pub low_pressure_bar: f64,
    pub clutch_engaged: bool,
    pub refrigerant_level_pct: f64,
    pub outlet_temp_c: f64,
    pub rpm: f64,
}

impl Default for AcCompressor {
    fn default() -> Self {
        Self::new()
    }
}

impl AcCompressor {
    pub fn new() -> Self {
        Self {
            high_pressure_bar: 15.0,
            low_pressure_bar: 2.5,
            clutch_engaged: true,
            refrigerant_level_pct: 90.0,
            outlet_temp_c: 5.0,
            rpm: 2000.0,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        self.high_pressure_bar > 10.0
            && self.high_pressure_bar < 25.0
            && self.low_pressure_bar > 1.5
            && self.low_pressure_bar < 4.0
    }

    pub fn refrigerant_ok(&self) -> bool {
        self.refrigerant_level_pct > 60.0
    }

    pub fn cooling_ok(&self) -> bool {
        self.outlet_temp_c < 10.0
    }

    pub fn needs_recharge(&self) -> bool {
        self.refrigerant_level_pct < 50.0
    }

    pub fn needs_service(&self) -> bool {
        !self.pressure_ok() || self.needs_recharge()
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.pressure_ok() {
            score -= 30.0;
        }
        if !self.refrigerant_ok() {
            score -= 25.0;
        }
        if !self.cooling_ok() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_ok() {
        let a = AcCompressor::new();
        assert!(a.pressure_ok());
    }

    #[test]
    fn test_refrigerant_ok() {
        let a = AcCompressor::new();
        assert!(a.refrigerant_ok());
    }

    #[test]
    fn test_cooling_ok() {
        let a = AcCompressor::new();
        assert!(a.cooling_ok());
    }

    #[test]
    fn test_no_recharge() {
        let a = AcCompressor::new();
        assert!(!a.needs_recharge());
    }

    #[test]
    fn test_low_refrigerant() {
        let mut a = AcCompressor::new();
        a.refrigerant_level_pct = 30.0;
        assert!(a.needs_recharge());
    }

    #[test]
    fn test_health() {
        let a = AcCompressor::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
