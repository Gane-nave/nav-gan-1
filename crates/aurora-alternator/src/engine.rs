/// Alternator monitoring: voltage output, charging current, belt drive
/// Phase 173

#[derive(Debug, Clone)]
pub struct Alternator {
    pub voltage_v: f64,
    pub current_a: f64,
    pub rpm: f64,
    pub temp_c: f64,
    pub efficiency_pct: f64,
}

impl Default for Alternator {
    fn default() -> Self {
        Self::new()
    }
}

impl Alternator {
    pub fn new() -> Self {
        Self {
            voltage_v: 14.2,
            current_a: 50.0,
            rpm: 3000.0,
            temp_c: 80.0,
            efficiency_pct: 65.0,
        }
    }

    pub fn output_watts(&self) -> f64 {
        self.voltage_v * self.current_a
    }

    pub fn voltage_ok(&self) -> bool {
        (13.5..=14.8).contains(&self.voltage_v)
    }

    pub fn is_overheating(&self) -> bool {
        self.temp_c > 150.0
    }

    pub fn charging(&self) -> bool {
        self.voltage_v > 13.0 && self.current_a > 0.0
    }

    pub fn health_score(&self) -> f64 {
        let volt_s = if self.voltage_ok() { 40.0 } else { 15.0 };
        let temp_s = if self.is_overheating() { 10.0 } else { 30.0 };
        let eff_s = self.efficiency_pct / 100.0 * 30.0;
        volt_s + temp_s + eff_s
    }

    pub fn needs_attention(&self) -> bool {
        !self.voltage_ok() || self.is_overheating() || !self.charging()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voltage_ok() {
        let a = Alternator::new();
        assert!(a.voltage_ok());
    }

    #[test]
    fn test_voltage_low() {
        let mut a = Alternator::new();
        a.voltage_v = 12.0;
        assert!(!a.voltage_ok());
    }

    #[test]
    fn test_output_watts() {
        let a = Alternator::new();
        assert!(a.output_watts() > 600.0);
    }

    #[test]
    fn test_charging() {
        let a = Alternator::new();
        assert!(a.charging());
    }

    #[test]
    fn test_not_overheating() {
        let a = Alternator::new();
        assert!(!a.is_overheating());
    }

    #[test]
    fn test_health() {
        let a = Alternator::new();
        assert!(a.health_score() > 80.0);
    }

    #[test]
    fn test_no_attention() {
        let a = Alternator::new();
        assert!(!a.needs_attention());
    }
}
