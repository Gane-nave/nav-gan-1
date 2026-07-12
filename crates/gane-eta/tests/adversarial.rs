use gane_eta::predictor::*;

#[test]
fn zero_length_segment() {
    let mut p = EtaPredictor::new();
    let req = EtaRequest {
        segments: vec![RouteSegment {
            length_m: 0.0,
            free_flow_speed_kmh: 60.0,
            current_speed_kmh: 40.0,
            historical_speed_kmh: 45.0,
            congestion_factor: 0.0,
        }],
        departure_time_ms: 0,
        day_of_week: 0,
        use_historical: true,
    };
    let r = p.predict(&req);
    assert!(r.total_seconds >= 0.0);
}

#[test]
fn very_slow_speed() {
    let mut p = EtaPredictor::new();
    let req = EtaRequest {
        segments: vec![RouteSegment {
            length_m: 1000.0,
            free_flow_speed_kmh: 1.0,
            current_speed_kmh: 0.5,
            historical_speed_kmh: 0.5,
            congestion_factor: 1.0,
        }],
        departure_time_ms: 0,
        day_of_week: 0,
        use_historical: true,
    };
    let r = p.predict(&req);
    assert!(r.total_seconds > 0.0);
}

#[test]
fn many_segments() {
    let mut p = EtaPredictor::new();
    let segs: Vec<RouteSegment> = (0..100)
        .map(|i| RouteSegment {
            length_m: 500.0,
            free_flow_speed_kmh: 60.0,
            current_speed_kmh: 30.0 + (i as f64) * 0.3,
            historical_speed_kmh: 40.0,
            congestion_factor: 0.2,
        })
        .collect();
    let req = EtaRequest {
        segments: segs,
        departure_time_ms: 0,
        day_of_week: 3,
        use_historical: true,
    };
    let r = p.predict(&req);
    assert_eq!(r.segments_eta.len(), 100);
    assert!(r.total_distance_m > 49000.0);
}

#[test]
fn high_congestion() {
    let mut p = EtaPredictor::new();
    let req = EtaRequest {
        segments: vec![RouteSegment {
            length_m: 1000.0,
            free_flow_speed_kmh: 100.0,
            current_speed_kmh: 10.0,
            historical_speed_kmh: 15.0,
            congestion_factor: 2.0,
        }],
        departure_time_ms: 0,
        day_of_week: 0,
        use_historical: true,
    };
    let r = p.predict(&req);
    assert!(r.delay_seconds > 0.0);
}

#[test]
fn empty_route() {
    let mut p = EtaPredictor::new();
    let req = EtaRequest {
        segments: vec![],
        departure_time_ms: 0,
        day_of_week: 0,
        use_historical: true,
    };
    let r = p.predict(&req);
    assert_eq!(r.total_seconds, 0.0);
    assert_eq!(r.total_distance_m, 0.0);
}

#[test]
fn result_serializes() {
    let r = EtaResult {
        total_seconds: 120.0,
        total_distance_m: 2000.0,
        confidence: 0.85,
        segments_eta: vec![120.0],
        delay_seconds: 30.0,
    };
    let json = serde_json::to_string(&r).unwrap();
    assert!(json.contains("total_seconds"));
}

#[test]
fn weights_normalize() {
    let mut p = EtaPredictor::with_weights(100.0, 100.0);
    let req = EtaRequest {
        segments: vec![RouteSegment {
            length_m: 1000.0,
            free_flow_speed_kmh: 60.0,
            current_speed_kmh: 60.0,
            historical_speed_kmh: 60.0,
            congestion_factor: 0.0,
        }],
        departure_time_ms: 0,
        day_of_week: 0,
        use_historical: true,
    };
    let r = p.predict(&req);
    assert!(r.total_seconds > 0.0);
}

#[test]
fn prediction_count_increments() {
    let mut p = EtaPredictor::new();
    let req = EtaRequest {
        segments: vec![RouteSegment {
            length_m: 100.0,
            free_flow_speed_kmh: 60.0,
            current_speed_kmh: 60.0,
            historical_speed_kmh: 60.0,
            congestion_factor: 0.0,
        }],
        departure_time_ms: 0,
        day_of_week: 0,
        use_historical: false,
    };
    for i in 0..50 {
        p.predict(&req);
        assert_eq!(p.prediction_count(), (i + 1) as u64);
    }
}
