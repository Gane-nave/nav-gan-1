/// otel baggage: set, get, remove, propagate, log
/// Phase 1755

#[derive(Debug, Clone)]
pub struct OtelBaggage {
    pub set_ok: bool,
    pub get_ok: bool,
    pub remove_ok: bool,
    pub propagate_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelBaggage {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelBaggage {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            get_ok: true,
            remove_ok: true,
            propagate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.get_ok && self.remove_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.propagate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.get_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
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
        let c = OtelBaggage::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelBaggage::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelBaggage::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelBaggage::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelBaggage::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelBaggage::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
