/// compressor: engage, pressurize, regulate, unload, disengage
/// Phase 1154

#[derive(Debug, Clone)]
pub struct Compressor {
    pub engage_ok: bool,
    pub pressurize_ok: bool,
    pub regulate_ok: bool,
    pub unload_ok: bool,
    pub disengage_ok: bool,
}

impl Default for Compressor {
    fn default() -> Self {
        Self::new()
    }
}

impl Compressor {
    pub fn new() -> Self {
        Self {
            engage_ok: true,
            pressurize_ok: true,
            regulate_ok: true,
            unload_ok: true,
            disengage_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.engage_ok && self.pressurize_ok && self.regulate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.unload_ok && self.disengage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.engage_ok || !self.pressurize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.engage_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = Compressor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Compressor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Compressor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Compressor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Compressor::new();
        c.engage_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Compressor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
