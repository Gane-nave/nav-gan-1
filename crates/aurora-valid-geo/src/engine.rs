/// valid geo: coords, bounds, distance, contain, log
/// Phase 2084

#[derive(Debug, Clone)]
pub struct ValidGeo {
    pub coords_ok: bool,
    pub bounds_ok: bool,
    pub distance_ok: bool,
    pub contain_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidGeo {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidGeo {
    pub fn new() -> Self {
        Self {
            coords_ok: true,
            bounds_ok: true,
            distance_ok: true,
            contain_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.coords_ok && self.bounds_ok && self.distance_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.contain_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.coords_ok || !self.bounds_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.coords_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ValidGeo::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidGeo::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidGeo::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidGeo::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidGeo::new();
        c.coords_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidGeo::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
