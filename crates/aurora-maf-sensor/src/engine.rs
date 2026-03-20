/// maf sensor: measure, heat, clean, trim, check
/// Phase 1247

#[derive(Debug, Clone)]
pub struct MafSensor {
    pub measure_ok: bool,
    pub heat_ok: bool,
    pub clean_ok: bool,
    pub trim_ok: bool,
    pub check_ok: bool,
}

impl Default for MafSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl MafSensor {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            heat_ok: true,
            clean_ok: true,
            trim_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.heat_ok && self.clean_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.trim_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.heat_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MafSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MafSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MafSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MafSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MafSensor::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MafSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
