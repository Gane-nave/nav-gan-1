/// Calendar navigation: event, reminder, travel time, commute
/// Phase 911

#[derive(Debug, Clone)]
pub struct CalendarNav {
    pub event_ok: bool,
    pub reminder_ok: bool,
    pub travel_ok: bool,
    pub commute_ok: bool,
    pub sync_ok: bool,
}

impl Default for CalendarNav {
    fn default() -> Self {
        Self::new()
    }
}

impl CalendarNav {
    pub fn new() -> Self {
        Self {
            event_ok: true,
            reminder_ok: true,
            travel_ok: true,
            commute_ok: true,
            sync_ok: true,
        }
    }

    pub fn scheduling_ok(&self) -> bool {
        self.event_ok && self.reminder_ok && self.sync_ok
    }

    pub fn routing_ok(&self) -> bool {
        self.travel_ok && self.commute_ok
    }

    pub fn all_ok(&self) -> bool {
        self.scheduling_ok() && self.routing_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.sync_ok || !self.event_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sync_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduling() {
        let c = CalendarNav::new();
        assert!(c.scheduling_ok());
    }

    #[test]
    fn test_routing() {
        let c = CalendarNav::new();
        assert!(c.routing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CalendarNav::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = CalendarNav::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_sync() {
        let mut c = CalendarNav::new();
        c.sync_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = CalendarNav::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
