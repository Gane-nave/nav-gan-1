/// Road closure: detection, alternative, schedule, notify
/// Phase 922

#[derive(Debug, Clone)]
pub struct RoadClosure {
    pub detect_ok: bool,
    pub alt_ok: bool,
    pub schedule_ok: bool,
    pub notify_ok: bool,
    pub database_ok: bool,
}

impl Default for RoadClosure {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadClosure {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            alt_ok: true,
            schedule_ok: true,
            notify_ok: true,
            database_ok: true,
        }
    }

    pub fn awareness_ok(&self) -> bool {
        self.detect_ok && self.schedule_ok && self.database_ok
    }

    pub fn routing_ok(&self) -> bool {
        self.alt_ok && self.notify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.awareness_ok() && self.routing_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.database_ok || !self.detect_ok
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
    fn test_awareness() {
        let c = RoadClosure::new();
        assert!(c.awareness_ok());
    }

    #[test]
    fn test_routing() {
        let c = RoadClosure::new();
        assert!(c.routing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoadClosure::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = RoadClosure::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_database() {
        let mut c = RoadClosure::new();
        c.database_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = RoadClosure::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
