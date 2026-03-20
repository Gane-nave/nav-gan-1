/// Power inverter: DC-AC conversion, PWM control, IGBT monitoring
/// Phase 295

#[derive(Debug, Clone)]
pub struct Inverter {
    pub dc_voltage: f64,
    pub ac_voltage: f64,
    pub current_a: f64,
    pub frequency_hz: f64,
    pub efficiency_pct: f64,
    pub igbt_temp_c: f64,
}

impl Default for Inverter {
    fn default() -> Self {
        Self::new()
    }
}

impl Inverter {
    pub fn new() -> Self {
        Self {
            dc_voltage: 400.0,
            ac_voltage: 0.0,
            current_a: 0.0,
            frequency_hz: 0.0,
            efficiency_pct: 97.0,
            igbt_temp_c: 50.0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.frequency_hz > 0.0
    }

    pub fn voltage_ok(&self) -> bool {
        self.dc_voltage > 300.0 && self.dc_voltage < 500.0
    }

    pub fn overheating(&self) -> bool {
        self.igbt_temp_c > 150.0
    }

    pub fn power_kw(&self) -> f64 {
        self.ac_voltage * self.current_a * 1.732 / 1000.0
    }

    pub fn health_score(&self) -> f64 {
        if self.overheating() {
            return 0.0;
        }
        if !self.voltage_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_active() {
        let i = Inverter::new();
        assert!(!i.is_active());
    }

    #[test]
    fn test_voltage_ok() {
        let i = Inverter::new();
        assert!(i.voltage_ok());
    }

    #[test]
    fn test_not_hot() {
        let i = Inverter::new();
        assert!(!i.overheating());
    }

    #[test]
    fn test_zero_power() {
        let i = Inverter::new();
        assert!(i.power_kw() < 0.1);
    }

    #[test]
    fn test_active() {
        let mut i = Inverter::new();
        i.frequency_hz = 100.0;
        assert!(i.is_active());
    }

    #[test]
    fn test_health() {
        let i = Inverter::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
