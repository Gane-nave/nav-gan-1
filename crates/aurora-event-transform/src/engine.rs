/// aurora-event-transform: event transform
/// Phase 2578

#[derive(Debug, Clone)]
pub struct EventTransform {
    pub map_ok: bool,
    pub reduce_ok: bool,
    pub enrich_ok: bool,
    pub split_ok: bool,
    pub merge_ok: bool,
}

impl Default for EventTransform {
    fn default() -> Self {
        Self::new()
    }
}

impl EventTransform {
    pub fn new() -> Self {
        Self {
            map_ok: true,
            reduce_ok: true,
            enrich_ok: true,
            split_ok: true,
            merge_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.map_ok && self.reduce_ok && self.enrich_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.split_ok && self.merge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.map_ok || !self.reduce_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.map_ok {
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
        let c = EventTransform::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventTransform::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventTransform::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventTransform::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventTransform::new();
        c.map_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventTransform::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventTransform::default();
        assert!(c.all_ok());
    }
}
