/// BMS cell balancing: cell voltage monitoring, passive/active balancing
/// Phase 298

#[derive(Debug, Clone)]
pub struct BmsCell {
    pub cell_count: u16,
    pub min_voltage: f64,
    pub max_voltage: f64,
    pub avg_voltage: f64,
    pub balancing_active: bool,
    pub all_sensors_ok: bool,
}

impl Default for BmsCell {
    fn default() -> Self {
        Self::new()
    }
}

impl BmsCell {
    pub fn new() -> Self {
        Self {
            cell_count: 96,
            min_voltage: 3.7,
            max_voltage: 3.75,
            avg_voltage: 3.72,
            balancing_active: false,
            all_sensors_ok: true,
        }
    }

    pub fn voltage_delta(&self) -> f64 {
        self.max_voltage - self.min_voltage
    }

    pub fn balanced(&self) -> bool {
        self.voltage_delta() < 0.02
    }

    pub fn needs_balancing(&self) -> bool {
        self.voltage_delta() > 0.05
    }

    pub fn cell_voltage_ok(&self) -> bool {
        self.min_voltage > 2.8 && self.max_voltage < 4.25
    }

    pub fn health_score(&self) -> f64 {
        if !self.cell_voltage_ok() {
            return 0.0;
        }
        if !self.all_sensors_ok {
            return 40.0;
        }
        if self.needs_balancing() {
            return 70.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        let b = BmsCell::new();
        assert!(b.voltage_delta() < 0.1);
    }

    #[test]
    fn test_balanced() {
        let b = BmsCell::new();
        assert!(!b.balanced());
    }

    #[test]
    fn test_no_balance_needed() {
        let b = BmsCell::new();
        assert!(!b.needs_balancing());
    }

    #[test]
    fn test_voltage_ok() {
        let b = BmsCell::new();
        assert!(b.cell_voltage_ok());
    }

    #[test]
    fn test_imbalanced() {
        let mut b = BmsCell::new();
        b.max_voltage = 4.0;
        assert!(b.needs_balancing());
    }

    #[test]
    fn test_health() {
        let b = BmsCell::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
