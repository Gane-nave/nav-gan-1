/// Crankshaft sensor: reluctor, signal, gap
/// Phase 582

#[derive(Debug, Clone)]
pub struct CrankshaftSensor {
    pub signal_ok: bool,
    pub gap_mm: f64,
    pub max_gap_mm: f64,
    pub reluctor_ok: bool,
    pub calibrated: bool,
}

impl Default for CrankshaftSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CrankshaftSensor {
    pub fn new() -> Self {
        Self {
            signal_ok: true,
            gap_mm: 0.8,
            max_gap_mm: 2.0,
            reluctor_ok: true,
            calibrated: true,
        }
    }

    pub fn signal_valid(&self) -> bool {
        self.signal_ok && self.reluctor_ok
    }

    pub fn gap_ok(&self) -> bool {
        self.gap_mm < self.max_gap_mm
    }

    pub fn all_ok(&self) -> bool {
        self.signal_valid() && self.gap_ok() && self.calibrated
    }

    pub fn needs_service(&self) -> bool {
        !self.signal_ok || !self.reluctor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.signal_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal() {
        let c = CrankshaftSensor::new();
        assert!(c.signal_valid());
    }

    #[test]
    fn test_gap() {
        let c = CrankshaftSensor::new();
        assert!(c.gap_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CrankshaftSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CrankshaftSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_fail() {
        let mut c = CrankshaftSensor::new();
        c.signal_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CrankshaftSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
