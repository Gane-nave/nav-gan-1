/// Crankshaft position sensor: RPM measurement, TDC detection, timing reference
/// Phase 306

#[derive(Debug, Clone)]
pub struct CrankSensor {
    pub rpm: f64,
    pub signal_ok: bool,
    pub tooth_count: u8,
    pub missing_teeth: u8,
    pub sync_achieved: bool,
}

impl Default for CrankSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CrankSensor {
    pub fn new() -> Self {
        Self {
            rpm: 800.0,
            signal_ok: true,
            tooth_count: 60,
            missing_teeth: 2,
            sync_achieved: true,
        }
    }

    pub fn engine_running(&self) -> bool {
        self.rpm > 200.0 && self.signal_ok
    }

    pub fn idle(&self) -> bool {
        self.rpm > 600.0 && self.rpm < 1000.0
    }

    pub fn over_rev(&self) -> bool {
        self.rpm > 7000.0
    }

    pub fn synchronized(&self) -> bool {
        self.sync_achieved && self.signal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.signal_ok {
            return 0.0;
        }
        if !self.sync_achieved {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running() {
        let c = CrankSensor::new();
        assert!(c.engine_running());
    }

    #[test]
    fn test_idle() {
        let c = CrankSensor::new();
        assert!(c.idle());
    }

    #[test]
    fn test_no_over_rev() {
        let c = CrankSensor::new();
        assert!(!c.over_rev());
    }

    #[test]
    fn test_synced() {
        let c = CrankSensor::new();
        assert!(c.synchronized());
    }

    #[test]
    fn test_high_rpm() {
        let mut c = CrankSensor::new();
        c.rpm = 8000.0;
        assert!(c.over_rev());
    }

    #[test]
    fn test_health() {
        let c = CrankSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
