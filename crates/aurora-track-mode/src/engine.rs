/// Track mode: stability off, ABS sport, TC off, timer
/// Phase 948

#[derive(Debug, Clone)]
pub struct TrackMode {
    pub stability_ok: bool,
    pub abs_ok: bool,
    pub tc_ok: bool,
    pub timer_ok: bool,
    pub telemetry_ok: bool,
}

impl Default for TrackMode {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackMode {
    pub fn new() -> Self {
        Self {
            stability_ok: true,
            abs_ok: true,
            tc_ok: true,
            timer_ok: true,
            telemetry_ok: true,
        }
    }

    pub fn safety_ok(&self) -> bool {
        self.stability_ok && self.abs_ok
    }

    pub fn performance_ok(&self) -> bool {
        self.tc_ok && self.timer_ok && self.telemetry_ok
    }

    pub fn all_ok(&self) -> bool {
        self.safety_ok() && self.performance_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.stability_ok || !self.abs_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.stability_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety() {
        let c = TrackMode::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_performance() {
        let c = TrackMode::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrackMode::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = TrackMode::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_stability() {
        let mut c = TrackMode::new();
        c.stability_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = TrackMode::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
