/// Side window: tempered, regulator, auto up/down
/// Phase 782

#[derive(Debug, Clone)]
pub struct SideWindow {
    pub tempered_ok: bool,
    pub regulator_ok: bool,
    pub auto_ok: bool,
    pub seal_ok: bool,
    pub tint_ok: bool,
}

impl Default for SideWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl SideWindow {
    pub fn new() -> Self {
        Self {
            tempered_ok: true,
            regulator_ok: true,
            auto_ok: true,
            seal_ok: true,
            tint_ok: true,
        }
    }

    pub fn glass_ok(&self) -> bool {
        self.tempered_ok && self.tint_ok && self.seal_ok
    }

    pub fn mechanism_ok(&self) -> bool {
        self.regulator_ok && self.auto_ok
    }

    pub fn all_ok(&self) -> bool {
        self.glass_ok() && self.mechanism_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.regulator_ok || !self.tempered_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tempered_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glass() {
        let c = SideWindow::new();
        assert!(c.glass_ok());
    }

    #[test]
    fn test_mechanism() {
        let c = SideWindow::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SideWindow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SideWindow::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_reg() {
        let mut c = SideWindow::new();
        c.regulator_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SideWindow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
