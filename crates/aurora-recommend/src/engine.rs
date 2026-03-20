/// Recommendation: profile, similarity, rank, diversify, explain
/// Phase 1028

#[derive(Debug, Clone)]
pub struct Recommend {
    pub profile_ok: bool,
    pub similarity_ok: bool,
    pub rank_ok: bool,
    pub diversify_ok: bool,
    pub explain_ok: bool,
}

impl Default for Recommend {
    fn default() -> Self {
        Self::new()
    }
}

impl Recommend {
    pub fn new() -> Self {
        Self {
            profile_ok: true,
            similarity_ok: true,
            rank_ok: true,
            diversify_ok: true,
            explain_ok: true,
        }
    }

    pub fn matching_ok(&self) -> bool {
        self.profile_ok && self.similarity_ok && self.rank_ok
    }

    pub fn quality_ok(&self) -> bool {
        self.diversify_ok && self.explain_ok
    }

    pub fn all_ok(&self) -> bool {
        self.matching_ok() && self.quality_ok()
    }

    pub fn needs_data(&self) -> bool {
        !self.profile_ok || !self.similarity_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.profile_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching() {
        let c = Recommend::new();
        assert!(c.matching_ok());
    }

    #[test]
    fn test_quality() {
        let c = Recommend::new();
        assert!(c.quality_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Recommend::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_data() {
        let c = Recommend::new();
        assert!(!c.needs_data());
    }

    #[test]
    fn test_profile() {
        let mut c = Recommend::new();
        c.profile_ok = false;
        assert!(c.needs_data());
    }

    #[test]
    fn test_health() {
        let c = Recommend::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
