/// speed limit: detect, parse, display, warn, enforce
/// Phase 1334

#[derive(Debug, Clone)]
pub struct SpeedLimit {
    pub detect_ok: bool,
    pub parse_ok: bool,
    pub display_ok: bool,
    pub warn_ok: bool,
    pub enforce_ok: bool,
}

impl Default for SpeedLimit {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeedLimit {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            parse_ok: true,
            display_ok: true,
            warn_ok: true,
            enforce_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.parse_ok && self.display_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.warn_ok && self.enforce_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.parse_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = SpeedLimit::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SpeedLimit::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SpeedLimit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SpeedLimit::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SpeedLimit::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SpeedLimit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
