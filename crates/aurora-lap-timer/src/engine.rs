/// Lap timer: sector, best, delta, history, prediction
/// Phase 950

#[derive(Debug, Clone)]
pub struct LapTimer {
    pub sector_ok: bool,
    pub best_ok: bool,
    pub delta_ok: bool,
    pub history_ok: bool,
    pub predict_ok: bool,
}

impl Default for LapTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl LapTimer {
    pub fn new() -> Self {
        Self {
            sector_ok: true,
            best_ok: true,
            delta_ok: true,
            history_ok: true,
            predict_ok: true,
        }
    }

    pub fn timing_ok(&self) -> bool {
        self.sector_ok && self.best_ok && self.delta_ok
    }

    pub fn analysis_ok(&self) -> bool {
        self.history_ok && self.predict_ok
    }

    pub fn all_ok(&self) -> bool {
        self.timing_ok() && self.analysis_ok()
    }

    pub fn needs_reset(&self) -> bool {
        !self.sector_ok || !self.best_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sector_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing() {
        let c = LapTimer::new();
        assert!(c.timing_ok());
    }

    #[test]
    fn test_analysis() {
        let c = LapTimer::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LapTimer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let c = LapTimer::new();
        assert!(!c.needs_reset());
    }

    #[test]
    fn test_sector() {
        let mut c = LapTimer::new();
        c.sector_ok = false;
        assert!(c.needs_reset());
    }

    #[test]
    fn test_health() {
        let c = LapTimer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
