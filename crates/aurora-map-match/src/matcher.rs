//! Core map-matching algorithm.

use serde::{Deserialize, Serialize};

/// A raw GPS observation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsObservation {
    pub lat: f64,
    pub lon: f64,
    pub accuracy_m: f64,
    pub timestamp_ms: u64,
    pub speed_mps: f64,
    pub heading_deg: f64,
}

/// A road segment candidate for matching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadSegment {
    pub id: u64,
    pub start_lat: f64,
    pub start_lon: f64,
    pub end_lat: f64,
    pub end_lon: f64,
    pub road_class: RoadClass,
    pub speed_limit_kmh: u32,
    pub name: String,
    pub one_way: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoadClass {
    Motorway,
    Trunk,
    Primary,
    Secondary,
    Tertiary,
    Residential,
    Service,
    Unclassified,
}

/// Result of map-matching a GPS point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub segment_id: u64,
    pub snapped_lat: f64,
    pub snapped_lon: f64,
    pub distance_from_road_m: f64,
    pub confidence: f64,
    pub road_name: String,
    pub road_class: RoadClass,
}

/// Map matching engine using HMM-based approach.
pub struct MapMatcher {
    segments: Vec<RoadSegment>,
    max_distance_m: f64,
    history: Vec<MatchResult>,
    sigma_z: f64,
}

impl MapMatcher {
    /// Create a new map matcher.
    pub fn new(max_distance_m: f64) -> Self {
        Self {
            segments: Vec::new(),
            max_distance_m,
            history: Vec::new(),
            sigma_z: 4.07, // GPS noise standard deviation
        }
    }

    /// Load road segments into the matcher.
    pub fn load_segments(&mut self, segments: Vec<RoadSegment>) {
        self.segments = segments;
    }

    /// Get the number of loaded road segments.
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Match a GPS observation to the nearest road segment.
    pub fn match_point(&mut self, obs: &GpsObservation) -> Option<MatchResult> {
        if self.segments.is_empty() {
            return None;
        }

        let mut best: Option<(usize, f64, f64, f64)> = None;

        for (i, seg) in self.segments.iter().enumerate() {
            let (snap_lat, snap_lon, dist) = Self::snap_to_segment(obs.lat, obs.lon, seg);

            if dist <= self.max_distance_m {
                let emission = Self::emission_prob(dist, self.sigma_z);

                let transition = if let Some(prev) = self.history.last() {
                    Self::transition_prob(prev.segment_id, seg.id)
                } else {
                    1.0
                };

                let score = emission * transition;

                if best.is_none() || score > best.unwrap().1 {
                    best = Some((i, score, snap_lat, snap_lon));
                }
            }
        }

        best.map(|(idx, score, snap_lat, snap_lon)| {
            let seg = &self.segments[idx];
            let dist = Self::haversine_m(obs.lat, obs.lon, snap_lat, snap_lon);
            let result = MatchResult {
                segment_id: seg.id,
                snapped_lat: snap_lat,
                snapped_lon: snap_lon,
                distance_from_road_m: dist,
                confidence: score.min(1.0),
                road_name: seg.name.clone(),
                road_class: seg.road_class,
            };
            self.history.push(result.clone());
            if self.history.len() > 100 {
                self.history.remove(0);
            }
            result
        })
    }

    /// Get match history.
    pub fn history(&self) -> &[MatchResult] {
        &self.history
    }

    /// Clear match history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    fn snap_to_segment(lat: f64, lon: f64, seg: &RoadSegment) -> (f64, f64, f64) {
        let dx = seg.end_lon - seg.start_lon;
        let dy = seg.end_lat - seg.start_lat;
        let len_sq = dx * dx + dy * dy;

        if len_sq < 1e-12 {
            let d = Self::haversine_m(lat, lon, seg.start_lat, seg.start_lon);
            return (seg.start_lat, seg.start_lon, d);
        }

        let t = ((lon - seg.start_lon) * dx + (lat - seg.start_lat) * dy) / len_sq;
        let t = t.clamp(0.0, 1.0);

        let snap_lat = seg.start_lat + t * dy;
        let snap_lon = seg.start_lon + t * dx;
        let d = Self::haversine_m(lat, lon, snap_lat, snap_lon);

        (snap_lat, snap_lon, d)
    }

    fn emission_prob(distance_m: f64, sigma: f64) -> f64 {
        (-0.5 * (distance_m / sigma).powi(2)).exp()
    }

    fn transition_prob(prev_id: u64, curr_id: u64) -> f64 {
        if prev_id == curr_id {
            1.0
        } else {
            0.8
        }
    }

    fn haversine_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6_371_000.0;
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        2.0 * r * a.sqrt().asin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_segment() -> RoadSegment {
        RoadSegment {
            id: 1,
            start_lat: 32.0850,
            start_lon: 34.7810,
            end_lat: 32.0860,
            end_lon: 34.7830,
            road_class: RoadClass::Primary,
            speed_limit_kmh: 50,
            name: "Dizengoff St".into(),
            one_way: false,
        }
    }

    #[test]
    fn new_matcher_empty() {
        let m = MapMatcher::new(50.0);
        assert_eq!(m.segment_count(), 0);
    }

    #[test]
    fn load_segments() {
        let mut m = MapMatcher::new(50.0);
        m.load_segments(vec![sample_segment()]);
        assert_eq!(m.segment_count(), 1);
    }

    #[test]
    fn match_point_near_road() {
        let mut m = MapMatcher::new(100.0);
        m.load_segments(vec![sample_segment()]);
        let obs = GpsObservation {
            lat: 32.0855,
            lon: 34.7820,
            accuracy_m: 5.0,
            timestamp_ms: 1000,
            speed_mps: 10.0,
            heading_deg: 45.0,
        };
        let result = m.match_point(&obs).unwrap();
        assert!(result.distance_from_road_m < 100.0);
        assert_eq!(result.road_name, "Dizengoff St");
    }

    #[test]
    fn match_point_no_segments() {
        let mut m = MapMatcher::new(50.0);
        let obs = GpsObservation {
            lat: 32.0,
            lon: 34.0,
            accuracy_m: 5.0,
            timestamp_ms: 1000,
            speed_mps: 0.0,
            heading_deg: 0.0,
        };
        assert!(m.match_point(&obs).is_none());
    }

    #[test]
    fn match_point_too_far() {
        let mut m = MapMatcher::new(1.0); // very small threshold
        m.load_segments(vec![sample_segment()]);
        let obs = GpsObservation {
            lat: 33.0,
            lon: 35.0,
            accuracy_m: 5.0,
            timestamp_ms: 1000,
            speed_mps: 0.0,
            heading_deg: 0.0,
        };
        assert!(m.match_point(&obs).is_none());
    }

    #[test]
    fn history_tracked() {
        let mut m = MapMatcher::new(100.0);
        m.load_segments(vec![sample_segment()]);
        let obs = GpsObservation {
            lat: 32.0855,
            lon: 34.7820,
            accuracy_m: 5.0,
            timestamp_ms: 1000,
            speed_mps: 10.0,
            heading_deg: 45.0,
        };
        m.match_point(&obs);
        assert_eq!(m.history().len(), 1);
        m.clear_history();
        assert_eq!(m.history().len(), 0);
    }

    #[test]
    fn road_class_variants() {
        let classes = [
            RoadClass::Motorway,
            RoadClass::Trunk,
            RoadClass::Primary,
            RoadClass::Secondary,
            RoadClass::Tertiary,
            RoadClass::Residential,
            RoadClass::Service,
            RoadClass::Unclassified,
        ];
        assert_eq!(classes.len(), 8);
    }

    #[test]
    fn haversine_zero_distance() {
        let d = MapMatcher::haversine_m(32.0, 34.0, 32.0, 34.0);
        assert!(d < 0.001);
    }
}
