/// humid sensor: measure, dewpoint, fog, defog, log
/// Phase 1295

#[derive(Debug, Clone)]
pub struct HumidSensor {
    pub measure_ok: bool,
    pub dewpoint_ok: bool,
    pub fog_ok: bool,
    pub defog_ok: bool,
    pub log_ok: bool,
}

impl Default for HumidSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl HumidSensor {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            dewpoint_ok: true,
            fog_ok: true,
            defog_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.dewpoint_ok && self.fog_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.defog_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.dewpoint_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = HumidSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HumidSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HumidSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HumidSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HumidSensor::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HumidSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
