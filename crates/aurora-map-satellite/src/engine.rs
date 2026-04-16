/// map satellite: fetch, mosaic, enhance, overlay, log
/// Phase 1441

#[derive(Debug, Clone)]
pub struct MapSatellite {
    pub fetch_ok: bool,
    pub mosaic_ok: bool,
    pub enhance_ok: bool,
    pub overlay_ok: bool,
    pub log_ok: bool,
}

impl Default for MapSatellite {
    fn default() -> Self {
        Self::new()
    }
}

impl MapSatellite {
    pub fn new() -> Self {
        Self {
            fetch_ok: true,
            mosaic_ok: true,
            enhance_ok: true,
            overlay_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fetch_ok && self.mosaic_ok && self.enhance_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.overlay_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fetch_ok || !self.mosaic_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fetch_ok {
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
        let c = MapSatellite::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapSatellite::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapSatellite::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapSatellite::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapSatellite::new();
        c.fetch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapSatellite::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
