/// aurora-qa-mutate: qa mutate
/// Phase 2513

#[derive(Debug, Clone)]
pub struct QaMutate {
    pub mutant_ok: bool,
    pub kill_ok: bool,
    pub survive_ok: bool,
    pub report_ok: bool,
    pub score_ok: bool,
}

impl Default for QaMutate {
    fn default() -> Self {
        Self::new()
    }
}

impl QaMutate {
    pub fn new() -> Self {
        Self {
            mutant_ok: true,
            kill_ok: true,
            survive_ok: true,
            report_ok: true,
            score_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mutant_ok && self.kill_ok && self.survive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.score_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mutant_ok || !self.kill_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mutant_ok {
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
        let c = QaMutate::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QaMutate::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QaMutate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QaMutate::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QaMutate::new();
        c.mutant_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QaMutate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = QaMutate::default();
        assert!(c.all_ok());
    }
}
