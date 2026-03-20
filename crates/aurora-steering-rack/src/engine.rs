/// Steering rack: pinion, tie rod, boot, power assist
/// Phase 636

#[derive(Debug, Clone)]
pub struct SteeringRack {
    pub pinion_ok: bool,
    pub tie_rod_ok: bool,
    pub boot_ok: bool,
    pub power_assist_ok: bool,
    pub play_mm: f64,
}

impl Default for SteeringRack {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringRack {
    pub fn new() -> Self {
        Self {
            pinion_ok: true,
            tie_rod_ok: true,
            boot_ok: true,
            power_assist_ok: true,
            play_mm: 1.0,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.pinion_ok && self.tie_rod_ok
    }

    pub fn sealed_ok(&self) -> bool {
        self.boot_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.sealed_ok() && self.power_assist_ok && self.play_mm < 3.0
    }

    pub fn needs_service(&self) -> bool {
        !self.pinion_ok || !self.boot_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pinion_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = SteeringRack::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_sealed() {
        let c = SteeringRack::new();
        assert!(c.sealed_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringRack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SteeringRack::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_pinion() {
        let mut c = SteeringRack::new();
        c.pinion_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SteeringRack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
