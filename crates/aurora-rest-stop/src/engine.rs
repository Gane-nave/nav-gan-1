/// Rest stop finder: facility, rating, distance, amenity
/// Phase 917

#[derive(Debug, Clone)]
pub struct RestStop {
    pub facility_ok: bool,
    pub rating_ok: bool,
    pub distance_ok: bool,
    pub amenity_ok: bool,
    pub database_ok: bool,
}

impl Default for RestStop {
    fn default() -> Self {
        Self::new()
    }
}

impl RestStop {
    pub fn new() -> Self {
        Self {
            facility_ok: true,
            rating_ok: true,
            distance_ok: true,
            amenity_ok: true,
            database_ok: true,
        }
    }

    pub fn search_ok(&self) -> bool {
        self.facility_ok && self.distance_ok && self.database_ok
    }

    pub fn quality_ok(&self) -> bool {
        self.rating_ok && self.amenity_ok
    }

    pub fn all_ok(&self) -> bool {
        self.search_ok() && self.quality_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.database_ok || !self.facility_ok
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
    fn test_search() {
        let c = RestStop::new();
        assert!(c.search_ok());
    }

    #[test]
    fn test_quality() {
        let c = RestStop::new();
        assert!(c.quality_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RestStop::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = RestStop::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_database() {
        let mut c = RestStop::new();
        c.database_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = RestStop::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
