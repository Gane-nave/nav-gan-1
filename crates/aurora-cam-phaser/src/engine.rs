/// Camshaft phaser: variable valve timing, advance/retard, oil pressure
/// Phase 310

#[derive(Debug, Clone)]
pub struct CamPhaser {
    pub advance_deg: f64,
    pub target_deg: f64,
    pub oil_pressure_ok: bool,
    pub solenoid_ok: bool,
    pub bank: u8,
}

impl Default for CamPhaser {
    fn default() -> Self {
        Self::new()
    }
}

impl CamPhaser {
    pub fn new() -> Self {
        Self {
            advance_deg: 15.0,
            target_deg: 15.0,
            oil_pressure_ok: true,
            solenoid_ok: true,
            bank: 1,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.advance_deg - self.target_deg).abs() < 2.0
    }

    pub fn can_operate(&self) -> bool {
        self.oil_pressure_ok && self.solenoid_ok
    }

    pub fn position_error(&self) -> f64 {
        (self.advance_deg - self.target_deg).abs()
    }

    pub fn fully_advanced(&self) -> bool {
        self.advance_deg > 40.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.solenoid_ok {
            return 0.0;
        }
        if !self.oil_pressure_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_target() {
        let c = CamPhaser::new();
        assert!(c.at_target());
    }

    #[test]
    fn test_can_operate() {
        let c = CamPhaser::new();
        assert!(c.can_operate());
    }

    #[test]
    fn test_no_error() {
        let c = CamPhaser::new();
        assert!(c.position_error() < 0.1);
    }

    #[test]
    fn test_not_advanced() {
        let c = CamPhaser::new();
        assert!(!c.fully_advanced());
    }

    #[test]
    fn test_no_oil() {
        let mut c = CamPhaser::new();
        c.oil_pressure_ok = false;
        assert!(!c.can_operate());
    }

    #[test]
    fn test_health() {
        let c = CamPhaser::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
