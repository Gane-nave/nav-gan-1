/// Intent-aware navigation: implicit destination, habits, proactive suggestions.
#[derive(Debug, Clone)]
pub struct LocationPattern {
    pub lat: f64,
    pub lon: f64,
    pub label: String,
    pub visit_count: u32,
    pub avg_arrival_hour: f64,
}
#[derive(Debug, Clone)]
pub struct IntentPredictor {
    pub patterns: Vec<LocationPattern>,
    pub current_hour: f64,
    pub day_of_week: u8,
}
impl IntentPredictor {
    pub fn new(hour: f64, dow: u8) -> Self {
        Self {
            patterns: Vec::new(),
            current_hour: hour,
            day_of_week: dow,
        }
    }
    pub fn add_pattern(&mut self, p: LocationPattern) {
        self.patterns.push(p);
    }
    pub fn predict_destination(&self) -> Option<&LocationPattern> {
        if self.patterns.is_empty() {
            return None;
        }
        self.patterns.iter().max_by(|a, b| {
            let sa = a.visit_count as f64
                * (1.0 / (1.0 + (a.avg_arrival_hour - self.current_hour).abs()));
            let sb = b.visit_count as f64
                * (1.0 / (1.0 + (b.avg_arrival_hour - self.current_hour).abs()));
            sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
        })
    }
    pub fn prediction_confidence(&self) -> f64 {
        match self.predict_destination() {
            None => 0.0,
            Some(p) => {
                let freq = (p.visit_count as f64 / 30.0).min(1.0);
                let time_match =
                    (1.0 / (1.0 + (p.avg_arrival_hour - self.current_hour).abs())).min(1.0);
                (freq * 0.6 + time_match * 0.4).clamp(0.0, 1.0)
            }
        }
    }
    pub fn should_suggest(&self) -> bool {
        self.prediction_confidence() > 0.5
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let p = IntentPredictor::new(8.0, 1);
        assert!(p.predict_destination().is_none());
        assert_eq!(p.prediction_confidence(), 0.0);
    }
    #[test]
    fn test_predict() {
        let mut p = IntentPredictor::new(8.0, 1);
        p.add_pattern(LocationPattern {
            lat: 32.0,
            lon: 34.0,
            label: "Work".into(),
            visit_count: 20,
            avg_arrival_hour: 8.5,
        });
        p.add_pattern(LocationPattern {
            lat: 32.1,
            lon: 34.1,
            label: "Gym".into(),
            visit_count: 5,
            avg_arrival_hour: 18.0,
        });
        assert_eq!(p.predict_destination().unwrap().label, "Work");
    }
    #[test]
    fn test_confidence() {
        let mut p = IntentPredictor::new(8.0, 1);
        p.add_pattern(LocationPattern {
            lat: 32.0,
            lon: 34.0,
            label: "Work".into(),
            visit_count: 30,
            avg_arrival_hour: 8.0,
        });
        assert!(p.prediction_confidence() > 0.5);
        assert!(p.should_suggest());
    }
}
