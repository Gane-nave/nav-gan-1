/// inverter: convert, modulate, filter, sync, protect
/// Phase 1148

#[derive(Debug, Clone)]
pub struct Inverter {
    pub convert_ok: bool,
    pub modulate_ok: bool,
    pub filter_ok: bool,
    pub sync_ok: bool,
    pub protect_ok: bool,
}

impl Default for Inverter {
    fn default() -> Self {
        Self::new()
    }
}

impl Inverter {
    pub fn new() -> Self {
        Self {
            convert_ok: true,
            modulate_ok: true,
            filter_ok: true,
            sync_ok: true,
            protect_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.convert_ok && self.modulate_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sync_ok && self.protect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.convert_ok || !self.modulate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.convert_ok {
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
        let c = Inverter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Inverter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Inverter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Inverter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Inverter::new();
        c.convert_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Inverter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
