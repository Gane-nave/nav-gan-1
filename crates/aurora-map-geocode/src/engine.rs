/// map geocode: parse, resolve, verify, format, log
/// Phase 1433

#[derive(Debug, Clone)]
pub struct MapGeocode {
    pub parse_ok: bool,
    pub resolve_ok: bool,
    pub verify_ok: bool,
    pub format_ok: bool,
    pub log_ok: bool,
}

impl Default for MapGeocode {
    fn default() -> Self {
        Self::new()
    }
}

impl MapGeocode {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            resolve_ok: true,
            verify_ok: true,
            format_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.resolve_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.format_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.resolve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = MapGeocode::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapGeocode::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapGeocode::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapGeocode::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapGeocode::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapGeocode::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
