/// Rear window: tempered glass, defroster, antenna
/// Phase 781

#[derive(Debug, Clone)]
pub struct RearWindow {
    pub tempered_ok: bool,
    pub defroster_ok: bool,
    pub antenna_ok: bool,
    pub tint_ok: bool,
    pub seal_ok: bool,
}

impl Default for RearWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl RearWindow {
    pub fn new() -> Self {
        Self {
            tempered_ok: true,
            defroster_ok: true,
            antenna_ok: true,
            tint_ok: true,
            seal_ok: true,
        }
    }

    pub fn glass_ok(&self) -> bool {
        self.tempered_ok && self.tint_ok
    }

    pub fn features_ok(&self) -> bool {
        self.defroster_ok && self.antenna_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.glass_ok() && self.features_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.tempered_ok || !self.seal_ok
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
        let c = RearWindow::new();
        assert!(c.glass_ok());
    }

    #[test]
    fn test_features() {
        let c = RearWindow::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RearWindow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = RearWindow::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_tempered() {
        let mut c = RearWindow::new();
        c.tempered_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = RearWindow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
