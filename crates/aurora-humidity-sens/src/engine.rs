/// Humidity sensor: cabin humidity, fogging prevention, auto defog
/// Phase 448

#[derive(Debug, Clone)]
pub struct HumiditySens {
    pub humidity_pct: f64,
    pub fog_risk: bool,
    pub sensor_ok: bool,
    pub auto_defog: bool,
    pub dewpoint_c: f64,
}

impl Default for HumiditySens {
    fn default() -> Self {
        Self::new()
    }
}

impl HumiditySens {
    pub fn new() -> Self {
        Self {
            humidity_pct: 45.0,
            fog_risk: false,
            sensor_ok: true,
            auto_defog: true,
            dewpoint_c: 10.0,
        }
    }

    pub fn comfortable(&self) -> bool {
        self.humidity_pct > 30.0 && self.humidity_pct < 70.0
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok && !self.fog_risk
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok
    }

    pub fn fog_prevention_active(&self) -> bool {
        self.auto_defog && self.fog_risk
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comfortable() {
        let h = HumiditySens::new();
        assert!(h.comfortable());
    }

    #[test]
    fn test_all_ok() {
        let h = HumiditySens::new();
        assert!(h.all_ok());
    }

    #[test]
    fn test_no_service() {
        let h = HumiditySens::new();
        assert!(!h.needs_service());
    }

    #[test]
    fn test_no_fog() {
        let h = HumiditySens::new();
        assert!(!h.fog_prevention_active());
    }

    #[test]
    fn test_bad_sensor() {
        let mut h = HumiditySens::new();
        h.sensor_ok = false;
        assert!(h.needs_service());
    }

    #[test]
    fn test_health() {
        let h = HumiditySens::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
