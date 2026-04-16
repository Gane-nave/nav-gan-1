/// DPF temperature sensor: pre/post filter, delta T
/// Phase 595

#[derive(Debug, Clone)]
pub struct DpfTemp {
    pub pre_temp_c: f64,
    pub post_temp_c: f64,
    pub sensor_ok: bool,
    pub in_range: bool,
    pub wiring_ok: bool,
}

impl Default for DpfTemp {
    fn default() -> Self {
        Self::new()
    }
}

impl DpfTemp {
    pub fn new() -> Self {
        Self {
            pre_temp_c: 350.0,
            post_temp_c: 300.0,
            sensor_ok: true,
            in_range: true,
            wiring_ok: true,
        }
    }

    pub fn delta_ok(&self) -> bool {
        self.pre_temp_c > self.post_temp_c
    }

    pub fn sensors_ok(&self) -> bool {
        self.sensor_ok && self.in_range && self.wiring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.delta_ok() && self.sensors_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.sensor_ok || !self.wiring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        let c = DpfTemp::new();
        assert!(c.delta_ok());
    }

    #[test]
    fn test_sensors() {
        let c = DpfTemp::new();
        assert!(c.sensors_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DpfTemp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = DpfTemp::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_sensor() {
        let mut c = DpfTemp::new();
        c.sensor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = DpfTemp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
