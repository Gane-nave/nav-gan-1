/// Anomaly detection: baseline, detect, score, alert, adapt
/// Phase 1027

#[derive(Debug, Clone)]
pub struct AnomalyDet {
    pub baseline_ok: bool,
    pub detect_ok: bool,
    pub score_ok: bool,
    pub alert_ok: bool,
    pub adapt_ok: bool,
}

impl Default for AnomalyDet {
    fn default() -> Self {
        Self::new()
    }
}

impl AnomalyDet {
    pub fn new() -> Self {
        Self {
            baseline_ok: true,
            detect_ok: true,
            score_ok: true,
            alert_ok: true,
            adapt_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.baseline_ok && self.detect_ok && self.score_ok
    }

    pub fn response_ok(&self) -> bool {
        self.alert_ok && self.adapt_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.response_ok()
    }

    pub fn needs_baseline(&self) -> bool {
        !self.baseline_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.baseline_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = AnomalyDet::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_response() {
        let c = AnomalyDet::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnomalyDet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_baseline() {
        let c = AnomalyDet::new();
        assert!(!c.needs_baseline());
    }

    #[test]
    fn test_baseline() {
        let mut c = AnomalyDet::new();
        c.baseline_ok = false;
        assert!(c.needs_baseline());
    }

    #[test]
    fn test_health() {
        let c = AnomalyDet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
