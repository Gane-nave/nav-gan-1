/// data load2: batch, stream, upsert, verify, log
/// Phase 2204

#[derive(Debug, Clone)]
pub struct DataLoad2 {
    pub batch_ok: bool,
    pub stream_ok: bool,
    pub upsert_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for DataLoad2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataLoad2 {
    pub fn new() -> Self {
        Self {
            batch_ok: true,
            stream_ok: true,
            upsert_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.batch_ok && self.stream_ok && self.upsert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.batch_ok || !self.stream_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.batch_ok {
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
        let c = DataLoad2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataLoad2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataLoad2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataLoad2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataLoad2::new();
        c.batch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataLoad2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
