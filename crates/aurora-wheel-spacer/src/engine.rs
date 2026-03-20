/// Wheel spacer: thickness, hub-centric, stud, grade
/// Phase 808

#[derive(Debug, Clone)]
pub struct WheelSpacer {
    pub thickness_ok: bool,
    pub hub_centric_ok: bool,
    pub stud_ok: bool,
    pub grade_ok: bool,
    pub torque_ok: bool,
}

impl Default for WheelSpacer {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelSpacer {
    pub fn new() -> Self {
        Self {
            thickness_ok: true,
            hub_centric_ok: true,
            stud_ok: true,
            grade_ok: true,
            torque_ok: true,
        }
    }

    pub fn fitment_ok(&self) -> bool {
        self.thickness_ok && self.hub_centric_ok
    }

    pub fn fastening_ok(&self) -> bool {
        self.stud_ok && self.grade_ok && self.torque_ok
    }

    pub fn all_ok(&self) -> bool {
        self.fitment_ok() && self.fastening_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.hub_centric_ok || !self.torque_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hub_centric_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fitment() {
        let c = WheelSpacer::new();
        assert!(c.fitment_ok());
    }

    #[test]
    fn test_fastening() {
        let c = WheelSpacer::new();
        assert!(c.fastening_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelSpacer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = WheelSpacer::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_hub() {
        let mut c = WheelSpacer::new();
        c.hub_centric_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = WheelSpacer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
