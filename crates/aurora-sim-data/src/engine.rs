/// aurora-sim-data: sim data
/// Phase 2531

#[derive(Debug, Clone)]
pub struct SimData {
    pub generate_ok: bool,
    pub stream_ok: bool,
    pub batch_ok: bool,
    pub replay_ok: bool,
    pub validate_ok: bool,
}

impl Default for SimData {
    fn default() -> Self {
        Self::new()
    }
}

impl SimData {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            stream_ok: true,
            batch_ok: true,
            replay_ok: true,
            validate_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.stream_ok && self.batch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.replay_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.stream_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = SimData::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimData::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimData::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimData::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimData::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimData::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimData::default();
        assert!(c.all_ok());
    }
}
