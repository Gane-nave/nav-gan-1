/// valid number: range, precision, format, convert, log
/// Phase 2077

#[derive(Debug, Clone)]
pub struct ValidNumber {
    pub range_ok: bool,
    pub precision_ok: bool,
    pub format_ok: bool,
    pub convert_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidNumber {
    pub fn new() -> Self {
        Self {
            range_ok: true,
            precision_ok: true,
            format_ok: true,
            convert_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.range_ok && self.precision_ok && self.format_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.convert_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.range_ok || !self.precision_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.range_ok {
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
        let c = ValidNumber::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidNumber::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidNumber::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidNumber::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidNumber::new();
        c.range_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidNumber::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
