/// search rank: score, boost, decay, combine, log
/// Phase 1889

#[derive(Debug, Clone)]
pub struct SearchRank {
    pub score_ok: bool,
    pub boost_ok: bool,
    pub decay_ok: bool,
    pub combine_ok: bool,
    pub log_ok: bool,
}

impl Default for SearchRank {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchRank {
    pub fn new() -> Self {
        Self {
            score_ok: true,
            boost_ok: true,
            decay_ok: true,
            combine_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.score_ok && self.boost_ok && self.decay_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.combine_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.score_ok || !self.boost_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.score_ok {
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
        let c = SearchRank::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SearchRank::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SearchRank::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SearchRank::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SearchRank::new();
        c.score_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SearchRank::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
