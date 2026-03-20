/// map style: parse, apply, theme, layer, log
/// Phase 1429

#[derive(Debug, Clone)]
pub struct MapStyle {
    pub parse_ok: bool,
    pub apply_ok: bool,
    pub theme_ok: bool,
    pub layer_ok: bool,
    pub log_ok: bool,
}

impl Default for MapStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl MapStyle {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            apply_ok: true,
            theme_ok: true,
            layer_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.apply_ok && self.theme_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.layer_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.apply_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MapStyle::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapStyle::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapStyle::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapStyle::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapStyle::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapStyle::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
