/// codec arrow2: create, convert, compute, export, log
/// Phase 2031

#[derive(Debug, Clone)]
pub struct CodecArrow2 {
    pub create_ok: bool,
    pub convert_ok: bool,
    pub compute_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for CodecArrow2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CodecArrow2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            convert_ok: true,
            compute_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.convert_ok && self.compute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.convert_ok
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
        let c = CodecArrow2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CodecArrow2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CodecArrow2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CodecArrow2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CodecArrow2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CodecArrow2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
