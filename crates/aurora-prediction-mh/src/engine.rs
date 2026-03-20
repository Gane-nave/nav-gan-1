/// Multi-horizon prediction: micro (5-15min), macro (hours), events, seasonal.
#[derive(Debug, Clone, PartialEq)]
pub enum PredictionHorizon {
    Micro,
    Short,
    Medium,
    Long,
    Seasonal,
}
#[derive(Debug, Clone)]
pub struct TrafficPrediction {
    pub horizon: PredictionHorizon,
    pub segment_id: u64,
    pub predicted_load: f64,
    pub confidence: f64,
    pub horizon_minutes: u32,
}
impl TrafficPrediction {
    pub fn is_reliable(&self) -> bool {
        self.confidence > 0.6
    }
    pub fn decay_factor(&self) -> f64 {
        (1.0 / (1.0 + self.horizon_minutes as f64 / 60.0)).clamp(0.1, 1.0)
    }
    pub fn adjusted_confidence(&self) -> f64 {
        (self.confidence * self.decay_factor()).clamp(0.0, 1.0)
    }
}
#[derive(Debug, Clone)]
pub struct MultiHorizonPredictor {
    pub predictions: Vec<TrafficPrediction>,
}
impl Default for MultiHorizonPredictor {
    fn default() -> Self {
        Self::new()
    }
}
impl MultiHorizonPredictor {
    pub fn new() -> Self {
        Self {
            predictions: Vec::new(),
        }
    }
    pub fn add_prediction(&mut self, p: TrafficPrediction) {
        self.predictions.push(p);
    }
    pub fn best_for_segment(&self, seg: u64) -> Option<&TrafficPrediction> {
        self.predictions
            .iter()
            .filter(|p| p.segment_id == seg)
            .max_by(|a, b| {
                a.adjusted_confidence()
                    .partial_cmp(&b.adjusted_confidence())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
    pub fn by_horizon(&self, h: PredictionHorizon) -> Vec<&TrafficPrediction> {
        self.predictions.iter().filter(|p| p.horizon == h).collect()
    }
    pub fn overall_confidence(&self) -> f64 {
        if self.predictions.is_empty() {
            return 0.0;
        }
        self.predictions
            .iter()
            .map(|p| p.adjusted_confidence())
            .sum::<f64>()
            / self.predictions.len() as f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decay() {
        let p = TrafficPrediction {
            horizon: PredictionHorizon::Micro,
            segment_id: 1,
            predicted_load: 0.7,
            confidence: 0.9,
            horizon_minutes: 10,
        };
        assert!(p.decay_factor() > 0.5);
        assert!(p.adjusted_confidence() > 0.0);
    }
    #[test]
    fn test_long_decay() {
        let p = TrafficPrediction {
            horizon: PredictionHorizon::Long,
            segment_id: 1,
            predicted_load: 0.5,
            confidence: 0.8,
            horizon_minutes: 480,
        };
        assert!(p.decay_factor() < 0.2);
    }
    #[test]
    fn test_best() {
        let mut m = MultiHorizonPredictor::new();
        m.add_prediction(TrafficPrediction {
            horizon: PredictionHorizon::Micro,
            segment_id: 1,
            predicted_load: 0.7,
            confidence: 0.9,
            horizon_minutes: 5,
        });
        m.add_prediction(TrafficPrediction {
            horizon: PredictionHorizon::Long,
            segment_id: 1,
            predicted_load: 0.5,
            confidence: 0.8,
            horizon_minutes: 480,
        });
        assert_eq!(
            m.best_for_segment(1).unwrap().horizon,
            PredictionHorizon::Micro
        );
    }
    #[test]
    fn test_empty() {
        let m = MultiHorizonPredictor::new();
        assert_eq!(m.overall_confidence(), 0.0);
    }
}
