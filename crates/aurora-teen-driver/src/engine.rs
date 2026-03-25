/// Teen driver: speed alert, volume limit, DND, report
/// Phase 891

#[derive(Debug, Clone)]
pub struct TeenDriver {
    pub speed_ok: bool,
    pub volume_ok: bool,
    pub dnd_ok: bool,
    pub report_ok: bool,
    pub geo_ok: bool,
}

impl Default for TeenDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl TeenDriver {
    pub fn new() -> Self {
        Self {
            speed_ok: true,
            volume_ok: true,
            dnd_ok: true,
            report_ok: true,
            geo_ok: true,
        }
    }

    pub fn safety_ok(&self) -> bool {
        self.speed_ok && self.dnd_ok && self.geo_ok
    }

    pub fn monitoring_ok(&self) -> bool {
        self.volume_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.safety_ok() && self.monitoring_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.speed_ok || !self.dnd_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.speed_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety() {
        let c = TeenDriver::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_monitoring() {
        let c = TeenDriver::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TeenDriver::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = TeenDriver::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_speed() {
        let mut c = TeenDriver::new();
        c.speed_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = TeenDriver::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
