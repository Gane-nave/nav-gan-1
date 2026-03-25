/// otel prometheus: scrape, expose, register, collect, log
/// Phase 1766

#[derive(Debug, Clone)]
pub struct OtelPrometheus {
    pub scrape_ok: bool,
    pub expose_ok: bool,
    pub register_ok: bool,
    pub collect_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelPrometheus {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelPrometheus {
    pub fn new() -> Self {
        Self {
            scrape_ok: true,
            expose_ok: true,
            register_ok: true,
            collect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scrape_ok && self.expose_ok && self.register_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.collect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scrape_ok || !self.expose_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scrape_ok {
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
        let c = OtelPrometheus::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelPrometheus::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelPrometheus::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelPrometheus::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelPrometheus::new();
        c.scrape_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelPrometheus::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
