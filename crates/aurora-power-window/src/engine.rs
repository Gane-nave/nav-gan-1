/// Power window: motor, regulator, switch, seal
/// Phase 679

#[derive(Debug, Clone)]
pub struct PowerWindow {
    pub motor_ok: bool,
    pub regulator_ok: bool,
    pub switch_ok: bool,
    pub seal_ok: bool,
    pub auto_ok: bool,
}

impl Default for PowerWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerWindow {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            regulator_ok: true,
            switch_ok: true,
            seal_ok: true,
            auto_ok: true,
        }
    }

    pub fn drive_ok(&self) -> bool {
        self.motor_ok && self.regulator_ok
    }

    pub fn controls_ok(&self) -> bool {
        self.switch_ok && self.auto_ok
    }

    pub fn all_ok(&self) -> bool {
        self.drive_ok() && self.controls_ok() && self.seal_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.regulator_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive() {
        let c = PowerWindow::new();
        assert!(c.drive_ok());
    }

    #[test]
    fn test_controls() {
        let c = PowerWindow::new();
        assert!(c.controls_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PowerWindow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = PowerWindow::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = PowerWindow::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = PowerWindow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
