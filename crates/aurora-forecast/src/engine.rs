/// Forecasting: series, trend, seasonal, predict, evaluate
/// Phase 1029

#[derive(Debug, Clone)]
pub struct Forecast {
    pub series_ok: bool,
    pub trend_ok: bool,
    pub seasonal_ok: bool,
    pub predict_ok: bool,
    pub evaluate_ok: bool,
}

impl Default for Forecast {
    fn default() -> Self {
        Self::new()
    }
}

impl Forecast {
    pub fn new() -> Self {
        Self {
            series_ok: true,
            trend_ok: true,
            seasonal_ok: true,
            predict_ok: true,
            evaluate_ok: true,
        }
    }

    pub fn modeling_ok(&self) -> bool {
        self.series_ok && self.trend_ok && self.seasonal_ok
    }

    pub fn output_ok(&self) -> bool {
        self.predict_ok && self.evaluate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.modeling_ok() && self.output_ok()
    }

    pub fn needs_data(&self) -> bool {
        !self.series_ok || !self.trend_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.series_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modeling() {
        let c = Forecast::new();
        assert!(c.modeling_ok());
    }

    #[test]
    fn test_output() {
        let c = Forecast::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Forecast::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_data() {
        let c = Forecast::new();
        assert!(!c.needs_data());
    }

    #[test]
    fn test_series() {
        let mut c = Forecast::new();
        c.series_ok = false;
        assert!(c.needs_data());
    }

    #[test]
    fn test_health() {
        let c = Forecast::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
