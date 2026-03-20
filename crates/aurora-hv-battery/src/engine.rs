/// High-voltage battery pack: state of charge, voltage, current, thermal
/// Phase 297

#[derive(Debug, Clone)]
pub struct HvBattery {
    pub soc_pct: f64,
    pub voltage: f64,
    pub current_a: f64,
    pub temp_c: f64,
    pub capacity_kwh: f64,
    pub cycle_count: u32,
}

impl Default for HvBattery {
    fn default() -> Self {
        Self::new()
    }
}

impl HvBattery {
    pub fn new() -> Self {
        Self {
            soc_pct: 80.0,
            voltage: 400.0,
            current_a: 0.0,
            temp_c: 25.0,
            capacity_kwh: 75.0,
            cycle_count: 100,
        }
    }

    pub fn charging(&self) -> bool {
        self.current_a > 0.0
    }

    pub fn discharging(&self) -> bool {
        self.current_a < 0.0
    }

    pub fn soc_low(&self) -> bool {
        self.soc_pct < 15.0
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c > -10.0 && self.temp_c < 45.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.temp_ok() {
            return 20.0;
        }
        if self.soc_low() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_charging() {
        let h = HvBattery::new();
        assert!(!h.charging());
    }

    #[test]
    fn test_not_discharging() {
        let h = HvBattery::new();
        assert!(!h.discharging());
    }

    #[test]
    fn test_not_low() {
        let h = HvBattery::new();
        assert!(!h.soc_low());
    }

    #[test]
    fn test_temp_ok() {
        let h = HvBattery::new();
        assert!(h.temp_ok());
    }

    #[test]
    fn test_low_soc() {
        let mut h = HvBattery::new();
        h.soc_pct = 5.0;
        assert!(h.soc_low());
    }

    #[test]
    fn test_health() {
        let h = HvBattery::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
