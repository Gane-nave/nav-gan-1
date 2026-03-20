/// Event traffic: venue, crowd, parking, surge predict
/// Phase 923

#[derive(Debug, Clone)]
pub struct EventTraffic {
    pub venue_ok: bool,
    pub crowd_ok: bool,
    pub parking_ok: bool,
    pub surge_ok: bool,
    pub calendar_ok: bool,
}

impl Default for EventTraffic {
    fn default() -> Self {
        Self::new()
    }
}

impl EventTraffic {
    pub fn new() -> Self {
        Self {
            venue_ok: true,
            crowd_ok: true,
            parking_ok: true,
            surge_ok: true,
            calendar_ok: true,
        }
    }

    pub fn prediction_ok(&self) -> bool {
        self.venue_ok && self.crowd_ok && self.calendar_ok
    }

    pub fn management_ok(&self) -> bool {
        self.parking_ok && self.surge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.prediction_ok() && self.management_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.calendar_ok || !self.venue_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.calendar_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction() {
        let c = EventTraffic::new();
        assert!(c.prediction_ok());
    }

    #[test]
    fn test_management() {
        let c = EventTraffic::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventTraffic::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = EventTraffic::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_calendar() {
        let mut c = EventTraffic::new();
        c.calendar_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = EventTraffic::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
