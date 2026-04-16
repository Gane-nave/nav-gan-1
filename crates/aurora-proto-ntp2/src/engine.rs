/// proto ntp2: sync, offset, drift, adjust, log
/// Phase 2011

#[derive(Debug, Clone)]
pub struct ProtoNtp2 {
    pub sync_ok: bool,
    pub offset_ok: bool,
    pub drift_ok: bool,
    pub adjust_ok: bool,
    pub log_ok: bool,
}

impl Default for ProtoNtp2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoNtp2 {
    pub fn new() -> Self {
        Self {
            sync_ok: true,
            offset_ok: true,
            drift_ok: true,
            adjust_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sync_ok && self.offset_ok && self.drift_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adjust_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sync_ok || !self.offset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sync_ok {
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
        let c = ProtoNtp2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ProtoNtp2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProtoNtp2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ProtoNtp2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ProtoNtp2::new();
        c.sync_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ProtoNtp2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
