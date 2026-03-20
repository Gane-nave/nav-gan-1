/// Alternator: charging voltage, current output, diode
/// Phase 516

#[derive(Debug, Clone)]
pub struct Alternator {
    pub voltage_v: f64,
    pub current_a: f64,
    pub max_current_a: f64,
    pub diode_ok: bool,
    pub bearing_ok: bool,
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
            current_a: 80.0,
            max_current_a: 150.0,
            diode_ok: true,
            bearing_ok: true,
        }
    }

    pub fn voltage_ok(&self) -> bool {
        self.voltage_v > 13.5 && self.voltage_v < 14.8
    }

    pub fn current_ok(&self) -> bool {
        self.current_a < self.max_current_a
    }

    pub fn all_ok(&self) -> bool {
        self.voltage_ok() && self.current_ok() && self.diode_ok && self.bearing_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.diode_ok || !self.bearing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.diode_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voltage() {
        let c = Alternator::new();
        assert!(c.voltage_ok());
    }

    #[test]
    fn test_current() {
        let c = Alternator::new();
        assert!(c.current_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Alternator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Alternator::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_diode() {
        let mut c = Alternator::new();
        c.diode_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Alternator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
