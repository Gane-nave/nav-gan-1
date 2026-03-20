/// wf notify2: send, template, batch, track, log
/// Phase 2234

#[derive(Debug, Clone)]
pub struct WfNotify2 {
    pub send_ok: bool,
    pub template_ok: bool,
    pub batch_ok: bool,
    pub track_ok: bool,
    pub log_ok: bool,
}

impl Default for WfNotify2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WfNotify2 {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            template_ok: true,
            batch_ok: true,
            track_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.template_ok && self.batch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.track_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.send_ok || !self.template_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.send_ok {
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
        let c = WfNotify2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfNotify2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfNotify2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfNotify2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfNotify2::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfNotify2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
