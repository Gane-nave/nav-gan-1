/// aurora-sim-route2: sim route2
/// Phase 2529

#[derive(Debug, Clone)]
pub struct SimRoute2 {
    pub generate_ok: bool,
    pub detour_ok: bool,
    pub block_ok: bool,
    pub clear_ok: bool,
    pub score_ok: bool,
}

impl Default for SimRoute2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SimRoute2 {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            detour_ok: true,
            block_ok: true,
            clear_ok: true,
            score_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.detour_ok && self.block_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clear_ok && self.score_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.detour_ok
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
        let c = SimRoute2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimRoute2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimRoute2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimRoute2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimRoute2::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimRoute2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimRoute2::default();
        assert!(c.all_ok());
    }
}
