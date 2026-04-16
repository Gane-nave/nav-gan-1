/// Daytime running lights: LED strip, intensity control, auto on/off
/// Phase 248

#[derive(Debug, Clone)]
pub struct DrlSystem {
    pub active: bool,
    pub intensity_pct: f64,
    pub left_ok: bool,
    pub right_ok: bool,
    pub auto_mode: bool,
    pub engine_running: bool,
}

impl Default for DrlSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl DrlSystem {
    pub fn new() -> Self {
        Self {
            active: true,
            intensity_pct: 100.0,
            left_ok: true,
            right_ok: true,
            auto_mode: true,
            engine_running: true,
        }
    }

    pub fn should_be_on(&self) -> bool {
        self.engine_running && self.auto_mode
    }

    pub fn both_ok(&self) -> bool {
        self.left_ok && self.right_ok
    }

    pub fn dimmed(&self) -> bool {
        self.intensity_pct < 80.0
    }

    pub fn needs_service(&self) -> bool {
        !self.left_ok || !self.right_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.left_ok && !self.right_ok {
            return 0.0;
        }
        if !self.both_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_be_on() {
        let d = DrlSystem::new();
        assert!(d.should_be_on());
    }

    #[test]
    fn test_both_ok() {
        let d = DrlSystem::new();
        assert!(d.both_ok());
    }

    #[test]
    fn test_not_dimmed() {
        let d = DrlSystem::new();
        assert!(!d.dimmed());
    }

    #[test]
    fn test_no_service() {
        let d = DrlSystem::new();
        assert!(!d.needs_service());
    }

    #[test]
    fn test_failure() {
        let mut d = DrlSystem::new();
        d.left_ok = false;
        assert!(d.needs_service());
    }

    #[test]
    fn test_health() {
        let d = DrlSystem::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
