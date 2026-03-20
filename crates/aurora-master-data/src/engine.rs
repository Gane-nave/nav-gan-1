/// Master data: entity, match, merge, golden, distribute
/// Phase 1040

#[derive(Debug, Clone)]
pub struct MasterData {
    pub entity_ok: bool,
    pub match_ok: bool,
    pub merge_ok: bool,
    pub golden_ok: bool,
    pub distribute_ok: bool,
}

impl Default for MasterData {
    fn default() -> Self {
        Self::new()
    }
}

impl MasterData {
    pub fn new() -> Self {
        Self {
            entity_ok: true,
            match_ok: true,
            merge_ok: true,
            golden_ok: true,
            distribute_ok: true,
        }
    }

    pub fn consolidation_ok(&self) -> bool {
        self.entity_ok && self.match_ok && self.merge_ok
    }

    pub fn distribution_ok(&self) -> bool {
        self.golden_ok && self.distribute_ok
    }

    pub fn all_ok(&self) -> bool {
        self.consolidation_ok() && self.distribution_ok()
    }

    pub fn needs_reconcile(&self) -> bool {
        !self.match_ok || !self.merge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.entity_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidation() {
        let c = MasterData::new();
        assert!(c.consolidation_ok());
    }

    #[test]
    fn test_distribution() {
        let c = MasterData::new();
        assert!(c.distribution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MasterData::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reconcile() {
        let c = MasterData::new();
        assert!(!c.needs_reconcile());
    }

    #[test]
    fn test_match() {
        let mut c = MasterData::new();
        c.match_ok = false;
        assert!(c.needs_reconcile());
    }

    #[test]
    fn test_health() {
        let c = MasterData::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
