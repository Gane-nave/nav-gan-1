/// Washer fluid level sensor: float switch, warning
/// Phase 696

#[derive(Debug, Clone)]
pub struct WasherLevel {
    pub float_ok: bool,
    pub switch_ok: bool,
    pub warning_ok: bool,
    pub connector_ok: bool,
    pub reading_ok: bool,
}

impl Default for WasherLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl WasherLevel {
    pub fn new() -> Self {
        Self {
            float_ok: true,
            switch_ok: true,
            warning_ok: true,
            connector_ok: true,
            reading_ok: true,
        }
    }

    pub fn sensor_ok(&self) -> bool {
        self.float_ok && self.switch_ok
    }

    pub fn output_ok(&self) -> bool {
        self.warning_ok && self.reading_ok && self.connector_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok() && self.output_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.float_ok || !self.switch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.float_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor() {
        let c = WasherLevel::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_output() {
        let c = WasherLevel::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WasherLevel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WasherLevel::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_float() {
        let mut c = WasherLevel::new();
        c.float_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WasherLevel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
