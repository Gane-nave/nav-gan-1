/// DC-DC converter: high voltage to 12V conversion, auxiliary power
/// Phase 296

#[derive(Debug, Clone)]
pub struct DcDcConverter {
    pub input_voltage: f64,
    pub output_voltage: f64,
    pub output_current_a: f64,
    pub efficiency_pct: f64,
    pub temp_c: f64,
    pub enabled: bool,
}

impl Default for DcDcConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl DcDcConverter {
    pub fn new() -> Self {
        Self {
            input_voltage: 400.0,
            output_voltage: 13.8,
            output_current_a: 20.0,
            efficiency_pct: 94.0,
            temp_c: 45.0,
            enabled: true,
        }
    }

    pub fn output_ok(&self) -> bool {
        self.output_voltage > 12.0 && self.output_voltage < 15.0
    }

    pub fn power_w(&self) -> f64 {
        self.output_voltage * self.output_current_a
    }

    pub fn overloaded(&self) -> bool {
        self.output_current_a > 100.0
    }

    pub fn overheating(&self) -> bool {
        self.temp_c > 100.0
    }

    pub fn health_score(&self) -> f64 {
        if self.overheating() {
            return 0.0;
        }
        if !self.output_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_ok() {
        let d = DcDcConverter::new();
        assert!(d.output_ok());
    }

    #[test]
    fn test_power() {
        let d = DcDcConverter::new();
        assert!(d.power_w() > 200.0);
    }

    #[test]
    fn test_not_overloaded() {
        let d = DcDcConverter::new();
        assert!(!d.overloaded());
    }

    #[test]
    fn test_not_hot() {
        let d = DcDcConverter::new();
        assert!(!d.overheating());
    }

    #[test]
    fn test_low_voltage() {
        let mut d = DcDcConverter::new();
        d.output_voltage = 10.0;
        assert!(!d.output_ok());
    }

    #[test]
    fn test_health() {
        let d = DcDcConverter::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
