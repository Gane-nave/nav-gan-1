/// Parking guidance: lot, space, price, navigate, reserve
/// Phase 924

#[derive(Debug, Clone)]
pub struct ParkingGuide {
    pub lot_ok: bool,
    pub space_ok: bool,
    pub price_ok: bool,
    pub navigate_ok: bool,
    pub reserve_ok: bool,
}

impl Default for ParkingGuide {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkingGuide {
    pub fn new() -> Self {
        Self {
            lot_ok: true,
            space_ok: true,
            price_ok: true,
            navigate_ok: true,
            reserve_ok: true,
        }
    }

    pub fn search_ok(&self) -> bool {
        self.lot_ok && self.space_ok && self.price_ok
    }

    pub fn booking_ok(&self) -> bool {
        self.navigate_ok && self.reserve_ok
    }

    pub fn all_ok(&self) -> bool {
        self.search_ok() && self.booking_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.lot_ok || !self.space_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lot_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let c = ParkingGuide::new();
        assert!(c.search_ok());
    }

    #[test]
    fn test_booking() {
        let c = ParkingGuide::new();
        assert!(c.booking_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ParkingGuide::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = ParkingGuide::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_lot() {
        let mut c = ParkingGuide::new();
        c.lot_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = ParkingGuide::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
