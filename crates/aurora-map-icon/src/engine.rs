/// map icon: load, scale, tint, place, log
/// Phase 1431

#[derive(Debug, Clone)]
pub struct MapIcon {
    pub load_ok: bool,
    pub scale_ok: bool,
    pub tint_ok: bool,
    pub place_ok: bool,
    pub log_ok: bool,
}

impl Default for MapIcon {
    fn default() -> Self {
        Self::new()
    }
}

impl MapIcon {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            scale_ok: true,
            tint_ok: true,
            place_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.scale_ok && self.tint_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.place_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.scale_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
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
        let c = MapIcon::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapIcon::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapIcon::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapIcon::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapIcon::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapIcon::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
