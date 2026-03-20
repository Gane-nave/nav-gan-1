use aurora_map_match::matcher::*;

fn road(id: u64, sy: f64, sx: f64, ey: f64, ex: f64) -> RoadSegment {
    RoadSegment {
        id,
        start_lat: sy,
        start_lon: sx,
        end_lat: ey,
        end_lon: ex,
        road_class: RoadClass::Primary,
        speed_limit_kmh: 50,
        name: format!("Road {id}"),
        one_way: false,
    }
}

#[test]
fn selects_closest_among_multiple_segments() {
    let mut m = MapMatcher::new(500.0);
    m.load_segments(vec![
        road(1, 32.085, 34.781, 32.086, 34.783),
        road(2, 32.090, 34.781, 32.091, 34.783),
    ]);
    let obs = GpsObservation {
        lat: 32.0855,
        lon: 34.782,
        accuracy_m: 5.0,
        timestamp_ms: 0,
        speed_mps: 0.0,
        heading_deg: 0.0,
    };
    let r = m.match_point(&obs).unwrap();
    assert_eq!(r.segment_id, 1);
}

#[test]
fn transition_favors_same_segment() {
    let mut m = MapMatcher::new(500.0);
    let seg = road(1, 32.085, 34.781, 32.090, 34.790);
    m.load_segments(vec![seg]);
    for i in 0..5 {
        let obs = GpsObservation {
            lat: 32.085 + (i as f64) * 0.001,
            lon: 34.782,
            accuracy_m: 5.0,
            timestamp_ms: i * 1000,
            speed_mps: 10.0,
            heading_deg: 0.0,
        };
        let r = m.match_point(&obs).unwrap();
        assert_eq!(r.segment_id, 1);
    }
}

#[test]
fn history_capped_at_100() {
    let mut m = MapMatcher::new(10000.0);
    m.load_segments(vec![road(1, 32.0, 34.0, 33.0, 35.0)]);
    for i in 0..150 {
        let obs = GpsObservation {
            lat: 32.0 + (i as f64) * 0.001,
            lon: 34.0,
            accuracy_m: 5.0,
            timestamp_ms: i * 100,
            speed_mps: 5.0,
            heading_deg: 0.0,
        };
        m.match_point(&obs);
    }
    assert!(m.history().len() <= 100);
}

#[test]
fn confidence_between_zero_and_one() {
    let mut m = MapMatcher::new(500.0);
    m.load_segments(vec![road(1, 32.085, 34.781, 32.086, 34.783)]);
    let obs = GpsObservation {
        lat: 32.0855,
        lon: 34.782,
        accuracy_m: 5.0,
        timestamp_ms: 0,
        speed_mps: 0.0,
        heading_deg: 0.0,
    };
    let r = m.match_point(&obs).unwrap();
    assert!((0.0..=1.0).contains(&r.confidence));
}

#[test]
fn snap_to_segment_endpoint() {
    let mut m = MapMatcher::new(500.0);
    m.load_segments(vec![road(1, 32.085, 34.781, 32.085, 34.781)]);
    let obs = GpsObservation {
        lat: 32.0851,
        lon: 34.7811,
        accuracy_m: 5.0,
        timestamp_ms: 0,
        speed_mps: 0.0,
        heading_deg: 0.0,
    };
    let r = m.match_point(&obs).unwrap();
    assert!(r.distance_from_road_m < 500.0);
}

#[test]
fn match_result_serializes() {
    let r = MatchResult {
        segment_id: 1,
        snapped_lat: 32.0,
        snapped_lon: 34.0,
        distance_from_road_m: 5.0,
        confidence: 0.9,
        road_name: "Test".into(),
        road_class: RoadClass::Primary,
    };
    let json = serde_json::to_string(&r).unwrap();
    assert!(json.contains("snapped_lat"));
}

#[test]
fn empty_segments_always_none() {
    let mut m = MapMatcher::new(f64::MAX);
    for _ in 0..10 {
        let obs = GpsObservation {
            lat: 0.0,
            lon: 0.0,
            accuracy_m: 1.0,
            timestamp_ms: 0,
            speed_mps: 0.0,
            heading_deg: 0.0,
        };
        assert!(m.match_point(&obs).is_none());
    }
}

#[test]
fn extreme_coordinates() {
    let mut m = MapMatcher::new(f64::MAX);
    m.load_segments(vec![road(1, -90.0, -180.0, 90.0, 180.0)]);
    let obs = GpsObservation {
        lat: 0.0,
        lon: 0.0,
        accuracy_m: 1.0,
        timestamp_ms: 0,
        speed_mps: 0.0,
        heading_deg: 0.0,
    };
    let r = m.match_point(&obs);
    assert!(r.is_some());
}
