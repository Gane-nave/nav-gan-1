use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtaRequest {
    pub segments: Vec<RouteSegment>,
    pub departure_time_ms: u64,
    pub day_of_week: u8,
    pub use_historical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSegment {
    pub length_m: f64,
    pub free_flow_speed_kmh: f64,
    pub current_speed_kmh: f64,
    pub historical_speed_kmh: f64,
    pub congestion_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtaResult {
    pub total_seconds: f64,
    pub total_distance_m: f64,
    pub confidence: f64,
    pub segments_eta: Vec<f64>,
    pub delay_seconds: f64,
}

pub struct EtaPredictor {
    historical_weight: f64,
    realtime_weight: f64,
    predictions: u64,
}

impl EtaPredictor {
    pub fn new() -> Self {
        Self {
            historical_weight: 0.3,
            realtime_weight: 0.7,
            predictions: 0,
        }
    }

    pub fn with_weights(historical: f64, realtime: f64) -> Self {
        let total = historical + realtime;
        Self {
            historical_weight: historical / total,
            realtime_weight: realtime / total,
            predictions: 0,
        }
    }

    pub fn predict(&mut self, req: &EtaRequest) -> EtaResult {
        let mut total_s = 0.0;
        let mut total_m = 0.0;
        let mut free_flow_s = 0.0;
        let mut seg_etas = Vec::with_capacity(req.segments.len());

        for seg in &req.segments {
            let effective_speed = if req.use_historical {
                seg.current_speed_kmh * self.realtime_weight
                    + seg.historical_speed_kmh * self.historical_weight
            } else {
                seg.current_speed_kmh
            };

            let speed_mps = (effective_speed.max(1.0) * 1000.0) / 3600.0;
            let seg_time = seg.length_m / speed_mps;
            let adjusted = seg_time * (1.0 + seg.congestion_factor * 0.5);

            seg_etas.push(adjusted);
            total_s += adjusted;
            total_m += seg.length_m;

            let ff_speed_mps = (seg.free_flow_speed_kmh.max(1.0) * 1000.0) / 3600.0;
            free_flow_s += seg.length_m / ff_speed_mps;
        }

        self.predictions += 1;

        EtaResult {
            total_seconds: total_s,
            total_distance_m: total_m,
            confidence: if req.use_historical { 0.85 } else { 0.7 },
            segments_eta: seg_etas,
            delay_seconds: (total_s - free_flow_s).max(0.0),
        }
    }

    pub fn prediction_count(&self) -> u64 {
        self.predictions
    }
}

impl Default for EtaPredictor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_req() -> EtaRequest {
        EtaRequest {
            segments: vec![RouteSegment {
                length_m: 1000.0,
                free_flow_speed_kmh: 60.0,
                current_speed_kmh: 40.0,
                historical_speed_kmh: 45.0,
                congestion_factor: 0.3,
            }],
            departure_time_ms: 0,
            day_of_week: 1,
            use_historical: true,
        }
    }

    #[test]
    fn new_predictor() {
        let p = EtaPredictor::new();
        assert_eq!(p.prediction_count(), 0);
    }
    #[test]
    fn default_impl() {
        let p = EtaPredictor::default();
        assert_eq!(p.prediction_count(), 0);
    }
    #[test]
    fn predict_simple() {
        let mut p = EtaPredictor::new();
        let r = p.predict(&simple_req());
        assert!(r.total_seconds > 0.0);
        assert!(r.total_distance_m > 0.0);
        assert_eq!(p.prediction_count(), 1);
    }
    #[test]
    fn predict_no_historical() {
        let mut p = EtaPredictor::new();
        let mut req = simple_req();
        req.use_historical = false;
        let r = p.predict(&req);
        assert!(r.confidence < 0.85);
    }
    #[test]
    fn delay_non_negative() {
        let mut p = EtaPredictor::new();
        let r = p.predict(&simple_req());
        assert!(r.delay_seconds >= 0.0);
    }
    #[test]
    fn segments_eta_count() {
        let mut p = EtaPredictor::new();
        let r = p.predict(&simple_req());
        assert_eq!(r.segments_eta.len(), 1);
    }
    #[test]
    fn custom_weights() {
        let p = EtaPredictor::with_weights(0.5, 0.5);
        assert_eq!(p.prediction_count(), 0);
    }
    #[test]
    fn multiple_predictions() {
        let mut p = EtaPredictor::new();
        for _ in 0..10 {
            p.predict(&simple_req());
        }
        assert_eq!(p.prediction_count(), 10);
    }
}
