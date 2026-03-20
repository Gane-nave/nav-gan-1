/// Error prevention: prevent wrong actions, detect confusion, correct before error.
#[derive(Debug, Clone, PartialEq)]
pub enum ConfusionIndicator {
    FrequentReroutes,
    SlowResponse,
    MissedTurns,
    ErraticSpeed,
    UturnsDetected,
}
#[derive(Debug, Clone)]
pub struct ErrorDetector {
    pub indicators: Vec<(ConfusionIndicator, f64)>,
    pub confusion_threshold: f64,
}
impl ErrorDetector {
    pub fn new(threshold: f64) -> Self {
        Self {
            indicators: Vec::new(),
            confusion_threshold: threshold,
        }
    }
    pub fn add_indicator(&mut self, ind: ConfusionIndicator, severity: f64) {
        let safe = if severity.is_finite() {
            severity.clamp(0.0, 1.0)
        } else {
            0.0
        };
        self.indicators.push((ind, safe));
    }
    pub fn confusion_score(&self) -> f64 {
        if self.indicators.is_empty() {
            0.0
        } else {
            self.indicators.iter().map(|(_, s)| s).sum::<f64>() / self.indicators.len() as f64
        }
    }
    pub fn is_confused(&self) -> bool {
        self.confusion_score() > self.confusion_threshold
    }
    pub fn worst_indicator(&self) -> Option<&ConfusionIndicator> {
        self.indicators
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
    }
}
#[derive(Debug, Clone)]
pub struct ActionValidator {
    pub valid_heading_range: (f64, f64),
    pub max_speed_mps: f64,
    pub min_distance_to_turn_m: f64,
}
impl ActionValidator {
    pub fn is_valid_heading(&self, heading: f64) -> bool {
        heading >= self.valid_heading_range.0 && heading <= self.valid_heading_range.1
    }
    pub fn is_safe_speed(&self, speed: f64) -> bool {
        speed <= self.max_speed_mps
    }
    pub fn can_make_turn(&self, distance: f64) -> bool {
        distance >= self.min_distance_to_turn_m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_confused() {
        let mut d = ErrorDetector::new(0.5);
        d.add_indicator(ConfusionIndicator::FrequentReroutes, 0.9);
        d.add_indicator(ConfusionIndicator::MissedTurns, 0.8);
        assert!(d.is_confused());
    }
    #[test]
    fn test_not_confused() {
        let mut d = ErrorDetector::new(0.5);
        d.add_indicator(ConfusionIndicator::SlowResponse, 0.2);
        assert!(!d.is_confused());
    }
    #[test]
    fn test_validator() {
        let v = ActionValidator {
            valid_heading_range: (80.0, 100.0),
            max_speed_mps: 33.0,
            min_distance_to_turn_m: 30.0,
        };
        assert!(v.is_valid_heading(90.0));
        assert!(!v.is_safe_speed(40.0));
        assert!(v.can_make_turn(50.0));
    }
    #[test]
    fn test_empty() {
        let d = ErrorDetector::new(0.5);
        assert_eq!(d.confusion_score(), 0.0);
        assert!(!d.is_confused());
    }
}
