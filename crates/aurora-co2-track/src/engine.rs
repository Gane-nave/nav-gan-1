/// CO2 tracking: emission, offset, report, regulation, goal
/// Phase 976

#[derive(Debug, Clone)]
pub struct Co2Track {
    pub emission_ok: bool,
    pub offset_ok: bool,
    pub report_ok: bool,
    pub regulation_ok: bool,
    pub goal_ok: bool,
}

impl Default for Co2Track {
    fn default() -> Self {
        Self::new()
    }
}

impl Co2Track {
    pub fn new() -> Self {
        Self {
            emission_ok: true,
            offset_ok: true,
            report_ok: true,
            regulation_ok: true,
            goal_ok: true,
        }
    }

    pub fn monitoring_ok(&self) -> bool {
        self.emission_ok && self.report_ok && self.regulation_ok
    }

    pub fn improvement_ok(&self) -> bool {
        self.offset_ok && self.goal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.monitoring_ok() && self.improvement_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.emission_ok || !self.regulation_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.emission_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring() {
        let c = Co2Track::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_improvement() {
        let c = Co2Track::new();
        assert!(c.improvement_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Co2Track::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = Co2Track::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_emission() {
        let mut c = Co2Track::new();
        c.emission_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = Co2Track::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
