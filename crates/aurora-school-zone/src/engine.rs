/// School zone: boundary, schedule, speed, alert
/// Phase 919

#[derive(Debug, Clone)]
pub struct SchoolZone {
    pub boundary_ok: bool,
    pub schedule_ok: bool,
    pub speed_ok: bool,
    pub alert_ok: bool,
    pub database_ok: bool,
}

impl Default for SchoolZone {
    fn default() -> Self {
        Self::new()
    }
}

impl SchoolZone {
    pub fn new() -> Self {
        Self {
            boundary_ok: true,
            schedule_ok: true,
            speed_ok: true,
            alert_ok: true,
            database_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.boundary_ok && self.schedule_ok && self.database_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.speed_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.safety_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.database_ok || !self.boundary_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.database_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = SchoolZone::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_safety() {
        let c = SchoolZone::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchoolZone::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = SchoolZone::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_database() {
        let mut c = SchoolZone::new();
        c.database_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = SchoolZone::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
