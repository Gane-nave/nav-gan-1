/// telemetry upload: collect, batch, compress, send, confirm
/// Phase 1134

#[derive(Debug, Clone)]
pub struct TelemUp {
    pub collect_ok: bool,
    pub batch_ok: bool,
    pub compress_ok: bool,
    pub send_ok: bool,
    pub confirm_ok: bool,
}

impl Default for TelemUp {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemUp {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            batch_ok: true,
            compress_ok: true,
            send_ok: true,
            confirm_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.batch_ok && self.compress_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.send_ok && self.confirm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.batch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TelemUp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TelemUp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TelemUp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TelemUp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TelemUp::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TelemUp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
