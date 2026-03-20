/// map charging: locate, filter, navigate, reserve, log
/// Phase 1445

#[derive(Debug, Clone)]
pub struct MapCharging {
    pub locate_ok: bool,
    pub filter_ok: bool,
    pub navigate_ok: bool,
    pub reserve_ok: bool,
    pub log_ok: bool,
}

impl Default for MapCharging {
    fn default() -> Self {
        Self::new()
    }
}

impl MapCharging {
    pub fn new() -> Self {
        Self {
            locate_ok: true,
            filter_ok: true,
            navigate_ok: true,
            reserve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.locate_ok && self.filter_ok && self.navigate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reserve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.locate_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.locate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MapCharging::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapCharging::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapCharging::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapCharging::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapCharging::new();
        c.locate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapCharging::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
