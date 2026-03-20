/// otel meter: create, counter, gauge, histogram, log
/// Phase 1762

#[derive(Debug, Clone)]
pub struct OtelMeter {
    pub create_ok: bool,
    pub counter_ok: bool,
    pub gauge_ok: bool,
    pub histogram_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelMeter {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelMeter {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            counter_ok: true,
            gauge_ok: true,
            histogram_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.counter_ok && self.gauge_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.histogram_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.counter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = OtelMeter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelMeter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelMeter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelMeter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelMeter::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelMeter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
