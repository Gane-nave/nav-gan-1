/// Efficiency coach: driving score, tips, route, habits
/// Phase 881

#[derive(Debug, Clone)]
pub struct EfficiencyCoach {
    pub score_ok: bool,
    pub tips_ok: bool,
    pub route_ok: bool,
    pub habits_ok: bool,
    pub feedback_ok: bool,
}

impl Default for EfficiencyCoach {
    fn default() -> Self {
        Self::new()
    }
}

impl EfficiencyCoach {
    pub fn new() -> Self {
        Self {
            score_ok: true,
            tips_ok: true,
            route_ok: true,
            habits_ok: true,
            feedback_ok: true,
        }
    }

    pub fn analysis_ok(&self) -> bool {
        self.score_ok && self.habits_ok && self.feedback_ok
    }

    pub fn guidance_ok(&self) -> bool {
        self.tips_ok && self.route_ok
    }

    pub fn all_ok(&self) -> bool {
        self.analysis_ok() && self.guidance_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.score_ok || !self.habits_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.score_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis() {
        let c = EfficiencyCoach::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_guidance() {
        let c = EfficiencyCoach::new();
        assert!(c.guidance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EfficiencyCoach::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = EfficiencyCoach::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_score() {
        let mut c = EfficiencyCoach::new();
        c.score_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = EfficiencyCoach::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
