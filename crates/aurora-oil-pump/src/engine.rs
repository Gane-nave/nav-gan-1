/// Oil pump: variable displacement, pressure regulation, flow control
/// Phase 302

#[derive(Debug, Clone)]
pub struct OilPump {
    pub pressure_bar: f64,
    pub min_pressure_bar: f64,
    pub flow_lpm: f64,
    pub oil_temp_c: f64,
    pub pump_ok: bool,
}

impl Default for OilPump {
    fn default() -> Self {
        Self::new()
    }
}

impl OilPump {
    pub fn new() -> Self {
        Self {
            pressure_bar: 3.5,
            min_pressure_bar: 1.0,
            flow_lpm: 15.0,
            oil_temp_c: 90.0,
            pump_ok: true,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        self.pressure_bar >= self.min_pressure_bar
    }

    pub fn oil_temp_ok(&self) -> bool {
        self.oil_temp_c > 20.0 && self.oil_temp_c < 130.0
    }

    pub fn low_pressure(&self) -> bool {
        self.pressure_bar < self.min_pressure_bar
    }

    pub fn overheating(&self) -> bool {
        self.oil_temp_c > 120.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.pump_ok {
            return 0.0;
        }
        if self.low_pressure() {
            return 20.0;
        }
        if self.overheating() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_ok() {
        let o = OilPump::new();
        assert!(o.pressure_ok());
    }

    #[test]
    fn test_temp_ok() {
        let o = OilPump::new();
        assert!(o.oil_temp_ok());
    }

    #[test]
    fn test_no_low() {
        let o = OilPump::new();
        assert!(!o.low_pressure());
    }

    #[test]
    fn test_no_overheating() {
        let o = OilPump::new();
        assert!(!o.overheating());
    }

    #[test]
    fn test_low() {
        let mut o = OilPump::new();
        o.pressure_bar = 0.5;
        assert!(o.low_pressure());
    }

    #[test]
    fn test_health() {
        let o = OilPump::new();
        assert!((o.health_score() - 100.0).abs() < 0.1);
    }
}
