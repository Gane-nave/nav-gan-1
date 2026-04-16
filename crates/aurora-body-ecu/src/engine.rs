/// Body ECU: door, window, mirror, lighting control
/// Phase 708

#[derive(Debug, Clone)]
pub struct BodyEcu {
    pub door_ctrl_ok: bool,
    pub window_ctrl_ok: bool,
    pub mirror_ctrl_ok: bool,
    pub light_ctrl_ok: bool,
    pub comm_ok: bool,
}

impl Default for BodyEcu {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyEcu {
    pub fn new() -> Self {
        Self {
            door_ctrl_ok: true,
            window_ctrl_ok: true,
            mirror_ctrl_ok: true,
            light_ctrl_ok: true,
            comm_ok: true,
        }
    }

    pub fn controls_ok(&self) -> bool {
        self.door_ctrl_ok && self.window_ctrl_ok && self.mirror_ctrl_ok
    }

    pub fn systems_ok(&self) -> bool {
        self.light_ctrl_ok && self.comm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.controls_ok() && self.systems_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.door_ctrl_ok || !self.comm_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.comm_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controls() {
        let c = BodyEcu::new();
        assert!(c.controls_ok());
    }

    #[test]
    fn test_systems() {
        let c = BodyEcu::new();
        assert!(c.systems_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BodyEcu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BodyEcu::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_comm() {
        let mut c = BodyEcu::new();
        c.comm_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BodyEcu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
