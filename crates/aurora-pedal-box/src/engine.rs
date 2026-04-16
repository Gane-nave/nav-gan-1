/// Pedal box: accelerator, brake, clutch, position sensor
/// Phase 751

#[derive(Debug, Clone)]
pub struct PedalBox {
    pub accel_ok: bool,
    pub brake_ok: bool,
    pub clutch_ok: bool,
    pub sensor_ok: bool,
    pub return_ok: bool,
}

impl Default for PedalBox {
    fn default() -> Self {
        Self::new()
    }
}

impl PedalBox {
    pub fn new() -> Self {
        Self {
            accel_ok: true,
            brake_ok: true,
            clutch_ok: true,
            sensor_ok: true,
            return_ok: true,
        }
    }

    pub fn pedals_ok(&self) -> bool {
        self.accel_ok && self.brake_ok && self.clutch_ok
    }

    pub fn feedback_ok(&self) -> bool {
        self.sensor_ok && self.return_ok
    }

    pub fn all_ok(&self) -> bool {
        self.pedals_ok() && self.feedback_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.brake_ok || !self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.brake_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pedals() {
        let c = PedalBox::new();
        assert!(c.pedals_ok());
    }

    #[test]
    fn test_feedback() {
        let c = PedalBox::new();
        assert!(c.feedback_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PedalBox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = PedalBox::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_brake() {
        let mut c = PedalBox::new();
        c.brake_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = PedalBox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
