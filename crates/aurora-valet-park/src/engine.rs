/// Valet parking: navigate, maneuver, park, retrieve, status
/// Phase 1119

#[derive(Debug, Clone)]
pub struct ValetPark {
    pub navigate_ok: bool,
    pub maneuver_ok: bool,
    pub park_ok: bool,
    pub retrieve_ok: bool,
    pub status_ok: bool,
}

impl Default for ValetPark {
    fn default() -> Self {
        Self::new()
    }
}

impl ValetPark {
    pub fn new() -> Self {
        Self {
            navigate_ok: true,
            maneuver_ok: true,
            park_ok: true,
            retrieve_ok: true,
            status_ok: true,
        }
    }

    pub fn parking_ok(&self) -> bool {
        self.navigate_ok && self.maneuver_ok && self.park_ok
    }

    pub fn service_ok(&self) -> bool {
        self.retrieve_ok && self.status_ok
    }

    pub fn all_ok(&self) -> bool {
        self.parking_ok() && self.service_ok()
    }

    pub fn needs_reset(&self) -> bool {
        !self.navigate_ok || !self.maneuver_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.navigate_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parking() {
        let c = ValetPark::new();
        assert!(c.parking_ok());
    }

    #[test]
    fn test_service() {
        let c = ValetPark::new();
        assert!(c.service_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValetPark::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let c = ValetPark::new();
        assert!(!c.needs_reset());
    }

    #[test]
    fn test_navigate() {
        let mut c = ValetPark::new();
        c.navigate_ok = false;
        assert!(c.needs_reset());
    }

    #[test]
    fn test_health() {
        let c = ValetPark::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
