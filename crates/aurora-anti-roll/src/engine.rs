/// anti roll: stiffen, soften, decouple, link, report
/// Phase 1197

#[derive(Debug, Clone)]
pub struct AntiRoll {
    pub stiffen_ok: bool,
    pub soften_ok: bool,
    pub decouple_ok: bool,
    pub link_ok: bool,
    pub report_ok: bool,
}

impl Default for AntiRoll {
    fn default() -> Self {
        Self::new()
    }
}

impl AntiRoll {
    pub fn new() -> Self {
        Self {
            stiffen_ok: true,
            soften_ok: true,
            decouple_ok: true,
            link_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.stiffen_ok && self.soften_ok && self.decouple_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.link_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.stiffen_ok || !self.soften_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.stiffen_ok {
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
        let c = AntiRoll::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AntiRoll::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AntiRoll::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AntiRoll::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AntiRoll::new();
        c.stiffen_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AntiRoll::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
