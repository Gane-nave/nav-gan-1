/// Differential temperature: oil temp, thermal protection, cooling
/// Phase 459

#[derive(Debug, Clone)]
pub struct DiffTemp {
    pub oil_temp_c: f64,
    pub max_temp_c: f64,
    pub oil_level_ok: bool,
    pub cooler_ok: bool,
    pub sensor_ok: bool,
}

impl Default for DiffTemp {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffTemp {
    pub fn new() -> Self {
        Self {
            oil_temp_c: 75.0,
            max_temp_c: 120.0,
            oil_level_ok: true,
            cooler_ok: true,
            sensor_ok: true,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.oil_temp_c < self.max_temp_c
    }

    pub fn all_ok(&self) -> bool {
        self.temp_ok() && self.oil_level_ok && self.sensor_ok
    }

    pub fn overheating(&self) -> bool {
        self.oil_temp_c > self.max_temp_c * 0.9
    }

    pub fn needs_service(&self) -> bool {
        !self.oil_level_ok || self.overheating()
    }

    pub fn health_score(&self) -> f64 {
        if !self.oil_level_ok {
            return 10.0;
        }
        if self.overheating() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp() {
        let d = DiffTemp::new();
        assert!(d.temp_ok());
    }

    #[test]
    fn test_all_ok() {
        let d = DiffTemp::new();
        assert!(d.all_ok());
    }

    #[test]
    fn test_no_overheat() {
        let d = DiffTemp::new();
        assert!(!d.overheating());
    }

    #[test]
    fn test_no_service() {
        let d = DiffTemp::new();
        assert!(!d.needs_service());
    }

    #[test]
    fn test_low_oil() {
        let mut d = DiffTemp::new();
        d.oil_level_ok = false;
        assert!(d.needs_service());
    }

    #[test]
    fn test_health() {
        let d = DiffTemp::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
