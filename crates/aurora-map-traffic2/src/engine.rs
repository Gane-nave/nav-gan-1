/// map traffic: fetch, decode, overlay, predict, log
/// Phase 1436

#[derive(Debug, Clone)]
pub struct MapTraffic2 {
    pub fetch_ok: bool,
    pub decode_ok: bool,
    pub overlay_ok: bool,
    pub predict_ok: bool,
    pub log_ok: bool,
}

impl Default for MapTraffic2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MapTraffic2 {
    pub fn new() -> Self {
        Self {
            fetch_ok: true,
            decode_ok: true,
            overlay_ok: true,
            predict_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fetch_ok && self.decode_ok && self.overlay_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.predict_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fetch_ok || !self.decode_ok
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
        let c = MapTraffic2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapTraffic2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapTraffic2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapTraffic2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapTraffic2::new();
        c.fetch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapTraffic2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
