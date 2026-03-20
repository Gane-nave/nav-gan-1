/// ac convert: rectify, invert, filter, regulate, log
/// Phase 1361

#[derive(Debug, Clone)]
pub struct AcConvert {
    pub rectify_ok: bool,
    pub invert_ok: bool,
    pub filter_ok: bool,
    pub regulate_ok: bool,
    pub log_ok: bool,
}

impl Default for AcConvert {
    fn default() -> Self {
        Self::new()
    }
}

impl AcConvert {
    pub fn new() -> Self {
        Self {
            rectify_ok: true,
            invert_ok: true,
            filter_ok: true,
            regulate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.rectify_ok && self.invert_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.regulate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.rectify_ok || !self.invert_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rectify_ok {
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
        let c = AcConvert::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AcConvert::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AcConvert::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AcConvert::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AcConvert::new();
        c.rectify_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AcConvert::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
