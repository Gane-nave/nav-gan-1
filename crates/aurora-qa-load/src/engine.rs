/// aurora-qa-load: qa load
/// Phase 2517

#[derive(Debug, Clone)]
pub struct QaLoad {
    pub ramp_ok: bool,
    pub sustain_ok: bool,
    pub spike_ok: bool,
    pub measure_ok: bool,
    pub report_ok: bool,
}

impl Default for QaLoad {
    fn default() -> Self {
        Self::new()
    }
}

impl QaLoad {
    pub fn new() -> Self {
        Self {
            ramp_ok: true,
            sustain_ok: true,
            spike_ok: true,
            measure_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.ramp_ok && self.sustain_ok && self.spike_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.measure_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.ramp_ok || !self.sustain_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ramp_ok {
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
        let c = QaLoad::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QaLoad::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QaLoad::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QaLoad::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QaLoad::new();
        c.ramp_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QaLoad::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = QaLoad::default();
        assert!(c.all_ok());
    }
}
