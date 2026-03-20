/// Traffic sign recognition and interpretation engine.
#[derive(Debug, Clone, PartialEq)]
pub enum SignType {
    SpeedLimit,
    Stop,
    Yield,
    NoEntry,
    OneWay,
    NoParking,
    NoUTurn,
    SchoolZone,
    Construction,
    Pedestrian,
    Warning,
    Information,
}

impl SignType {
    pub fn priority(&self) -> u8 {
        match self {
            SignType::Stop => 10,
            SignType::NoEntry => 9,
            SignType::SpeedLimit => 8,
            SignType::Yield => 7,
            SignType::SchoolZone => 7,
            SignType::Construction => 6,
            SignType::NoUTurn => 5,
            SignType::OneWay => 5,
            SignType::NoParking => 3,
            SignType::Pedestrian => 4,
            SignType::Warning => 4,
            SignType::Information => 1,
        }
    }

    pub fn is_regulatory(&self) -> bool {
        matches!(
            self,
            SignType::SpeedLimit
                | SignType::Stop
                | SignType::Yield
                | SignType::NoEntry
                | SignType::OneWay
                | SignType::NoParking
                | SignType::NoUTurn
        )
    }

    pub fn is_warning(&self) -> bool {
        matches!(
            self,
            SignType::Warning
                | SignType::Construction
                | SignType::SchoolZone
                | SignType::Pedestrian
        )
    }
}

#[derive(Debug, Clone)]
pub struct DetectedSign {
    pub sign_type: SignType,
    pub confidence: f64,
    pub distance_m: f64,
    pub value: Option<f64>,
}

impl DetectedSign {
    pub fn new(sign_type: SignType, confidence: f64, distance_m: f64) -> Self {
        Self {
            sign_type,
            confidence,
            distance_m,
            value: None,
        }
    }

    pub fn with_value(mut self, val: f64) -> Self {
        self.value = Some(val);
        self
    }

    pub fn is_reliable(&self) -> bool {
        self.confidence >= 0.8
    }

    pub fn time_to_reach_sec(&self, speed_kmh: f64) -> f64 {
        if speed_kmh <= 0.0 {
            return f64::INFINITY;
        }
        self.distance_m / (speed_kmh / 3.6)
    }

    pub fn requires_action(&self) -> bool {
        self.is_reliable() && self.sign_type.is_regulatory()
    }

    pub fn speed_limit(&self) -> Option<f64> {
        if self.sign_type == SignType::SpeedLimit {
            self.value
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct SignDetectionResult {
    pub signs: Vec<DetectedSign>,
}

impl Default for SignDetectionResult {
    fn default() -> Self {
        Self::new()
    }
}

impl SignDetectionResult {
    pub fn new() -> Self {
        Self { signs: Vec::new() }
    }

    pub fn add(&mut self, sign: DetectedSign) {
        self.signs.push(sign);
    }

    pub fn active_speed_limit(&self) -> Option<f64> {
        self.signs
            .iter()
            .filter(|s| s.sign_type == SignType::SpeedLimit && s.is_reliable())
            .filter_map(|s| s.value)
            .next()
    }

    pub fn has_stop(&self) -> bool {
        self.signs
            .iter()
            .any(|s| s.sign_type == SignType::Stop && s.is_reliable())
    }

    pub fn regulatory_count(&self) -> usize {
        self.signs
            .iter()
            .filter(|s| s.sign_type.is_regulatory() && s.is_reliable())
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.signs
            .iter()
            .filter(|s| s.sign_type.is_warning() && s.is_reliable())
            .count()
    }

    pub fn highest_priority(&self) -> Option<&DetectedSign> {
        self.signs
            .iter()
            .filter(|s| s.is_reliable())
            .max_by_key(|s| s.sign_type.priority())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_priority() {
        assert!(SignType::Stop.priority() > SignType::Information.priority());
    }

    #[test]
    fn test_regulatory() {
        assert!(SignType::Stop.is_regulatory());
        assert!(!SignType::Warning.is_regulatory());
    }

    #[test]
    fn test_warning() {
        assert!(SignType::Construction.is_warning());
        assert!(!SignType::Stop.is_warning());
    }

    #[test]
    fn test_reliable() {
        assert!(DetectedSign::new(SignType::Stop, 0.95, 50.0).is_reliable());
        assert!(!DetectedSign::new(SignType::Stop, 0.5, 50.0).is_reliable());
    }

    #[test]
    fn test_time_to_reach() {
        let s = DetectedSign::new(SignType::Stop, 0.9, 100.0);
        let t = s.time_to_reach_sec(72.0);
        assert!((t - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_speed_limit_value() {
        let s = DetectedSign::new(SignType::SpeedLimit, 0.9, 50.0).with_value(80.0);
        assert_eq!(s.speed_limit(), Some(80.0));
    }

    #[test]
    fn test_no_speed_limit() {
        let s = DetectedSign::new(SignType::Stop, 0.9, 50.0);
        assert_eq!(s.speed_limit(), None);
    }

    #[test]
    fn test_requires_action() {
        let s = DetectedSign::new(SignType::Stop, 0.95, 50.0);
        assert!(s.requires_action());
    }

    #[test]
    fn test_result_speed_limit() {
        let mut r = SignDetectionResult::new();
        r.add(DetectedSign::new(SignType::SpeedLimit, 0.9, 50.0).with_value(60.0));
        assert_eq!(r.active_speed_limit(), Some(60.0));
    }

    #[test]
    fn test_has_stop() {
        let mut r = SignDetectionResult::new();
        r.add(DetectedSign::new(SignType::Stop, 0.9, 30.0));
        assert!(r.has_stop());
    }

    #[test]
    fn test_regulatory_count() {
        let mut r = SignDetectionResult::new();
        r.add(DetectedSign::new(SignType::Stop, 0.9, 30.0));
        r.add(DetectedSign::new(SignType::Warning, 0.9, 50.0));
        assert_eq!(r.regulatory_count(), 1);
    }

    #[test]
    fn test_highest_priority() {
        let mut r = SignDetectionResult::new();
        r.add(DetectedSign::new(SignType::Information, 0.9, 100.0));
        r.add(DetectedSign::new(SignType::Stop, 0.9, 30.0));
        let hp = r.highest_priority().unwrap();
        assert_eq!(hp.sign_type, SignType::Stop);
    }
}
