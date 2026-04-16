/// Boost pressure sensor: turbo, wastegate, intercooler
/// Phase 593

#[derive(Debug, Clone)]
pub struct BoostSensor {
    pub boost_bar: f64,
    pub max_boost_bar: f64,
    pub wastegate_ok: bool,
    pub intercooler_ok: bool,
    pub sensor_ok: bool,
}

impl Default for BoostSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl BoostSensor {
    pub fn new() -> Self {
        Self {
            boost_bar: 1.2,
            max_boost_bar: 2.0,
            wastegate_ok: true,
            intercooler_ok: true,
            sensor_ok: true,
        }
    }

    pub fn boost_ok(&self) -> bool {
        self.boost_bar < self.max_boost_bar && self.boost_bar >= 0.0
    }

    pub fn system_ok(&self) -> bool {
        self.wastegate_ok && self.intercooler_ok && self.sensor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.boost_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.wastegate_ok || !self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wastegate_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boost() {
        let c = BoostSensor::new();
        assert!(c.boost_ok());
    }

    #[test]
    fn test_system() {
        let c = BoostSensor::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BoostSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BoostSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_wastegate() {
        let mut c = BoostSensor::new();
        c.wastegate_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BoostSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
