//! Map matching — snaps GNSS positions to the road network.

use gane_core::types::{EntityId, GeoPosition};
use tracing::{debug, info};

use crate::graph::RoadGraphIndex;

/// Match confidence level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchConfidence {
    /// High confidence — position clearly on a single segment.
    High,
    /// Medium — position near intersection or ambiguous.
    Medium,
    /// Low — large distance to nearest segment.
    Low,
    /// NoMatch — no road segment within search radius.
    NoMatch,
}

/// Result of a map-matching operation.
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// The matched road segment ID.
    pub segment_id: Option<EntityId>,
    /// The snapped position on the road.
    pub snapped_position: GeoPosition,
    /// Distance from raw position to snapped position (metres).
    pub offset_m: f64,
    /// Heading along the matched segment (degrees, 0=north).
    pub road_heading_deg: Option<f64>,
    /// Confidence of the match.
    pub confidence: MatchConfidence,
}

/// Map matcher that uses a hidden Markov model-inspired approach.
/// Maintains state across consecutive fixes for temporal consistency.
pub struct MapMatcher {
    /// Maximum distance to consider a match (metres).
    max_match_distance_m: f64,
    /// Previous match for temporal consistency.
    previous_match: Option<MatchResult>,
    /// Number of consecutive low-confidence matches.
    low_confidence_streak: u32,
    /// Total matches performed.
    total_matches: u64,
}

impl MapMatcher {
    pub fn new() -> Self {
        Self {
            max_match_distance_m: 50.0,
            previous_match: None,
            low_confidence_streak: 0,
            total_matches: 0,
        }
    }

    /// Set the maximum match distance.
    pub fn with_max_distance(mut self, distance_m: f64) -> Self {
        self.max_match_distance_m = distance_m;
        self
    }

    /// Match a raw GNSS position to the road network.
    pub fn match_position(
        &mut self,
        raw_position: &GeoPosition,
        index: &RoadGraphIndex,
    ) -> MatchResult {
        self.total_matches += 1;

        let result = match index.nearest_segment(raw_position) {
            Some((seg_id, proj, dist)) if dist <= self.max_match_distance_m => {
                let confidence = if dist < 5.0 {
                    MatchConfidence::High
                } else if dist < 20.0 {
                    MatchConfidence::Medium
                } else {
                    MatchConfidence::Low
                };

                // Compute road heading from segment geometry.
                let road_heading_deg = index.segment(&seg_id).and_then(|seg| {
                    if seg.geometry.len() >= 2 {
                        Some(bearing_deg(&seg.geometry[0], seg.geometry.last().unwrap()))
                    } else {
                        None
                    }
                });

                // Temporal consistency: if we had a previous match on the same segment,
                // boost confidence.
                let confidence = if let Some(prev) = &self.previous_match {
                    if prev.segment_id == Some(seg_id) && confidence == MatchConfidence::Medium {
                        MatchConfidence::High
                    } else {
                        confidence
                    }
                } else {
                    confidence
                };

                debug!(
                    segment = %seg_id,
                    offset_m = dist,
                    ?confidence,
                    "map matched"
                );

                MatchResult {
                    segment_id: Some(seg_id),
                    snapped_position: proj,
                    offset_m: dist,
                    road_heading_deg,
                    confidence,
                }
            }
            Some((_, _, dist)) => {
                debug!(dist, "nearest segment too far — no match");
                MatchResult {
                    segment_id: None,
                    snapped_position: *raw_position,
                    offset_m: dist,
                    road_heading_deg: None,
                    confidence: MatchConfidence::NoMatch,
                }
            }
            None => MatchResult {
                segment_id: None,
                snapped_position: *raw_position,
                offset_m: f64::MAX,
                road_heading_deg: None,
                confidence: MatchConfidence::NoMatch,
            },
        };

        // Track low-confidence streaks.
        match result.confidence {
            MatchConfidence::Low | MatchConfidence::NoMatch => {
                self.low_confidence_streak += 1;
                if self.low_confidence_streak >= 5 {
                    info!(
                        streak = self.low_confidence_streak,
                        "sustained low map-match confidence — possible off-road or map gap"
                    );
                }
            }
            _ => {
                self.low_confidence_streak = 0;
            }
        }

        self.previous_match = Some(result.clone());
        result
    }

    /// Reset the matcher state (e.g., after a reroute).
    pub fn reset(&mut self) {
        self.previous_match = None;
        self.low_confidence_streak = 0;
    }

    /// Total number of matches performed.
    pub fn total_matches(&self) -> u64 {
        self.total_matches
    }

    /// Current low-confidence streak length.
    pub fn low_confidence_streak(&self) -> u32 {
        self.low_confidence_streak
    }

    /// Last match result.
    pub fn last_match(&self) -> Option<&MatchResult> {
        self.previous_match.as_ref()
    }
}

impl Default for MapMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute bearing in degrees from point A to point B.
fn bearing_deg(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let dlon = (b.longitude_deg - a.longitude_deg).to_radians();

    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    let bearing = y.atan2(x).to_degrees();
    (bearing + 360.0) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use gane_core::map::*;

    fn make_index() -> RoadGraphIndex {
        let n1 = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.0853,
                longitude_deg: 34.7818,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        let n2 = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.0863,
                longitude_deg: 34.7818,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        let seg = RoadSegment {
            id: EntityId::new(),
            from_node: n1.id,
            to_node: n2.id,
            geometry: vec![n1.position, n2.position],
            road_class: RoadClass::Primary,
            one_way: false,
            speed_limit_kmh: Some(50.0),
            lane_count: Some(2),
            surface_type: SurfaceType::Asphalt,
            bridge: false,
            tunnel: false,
            toll: false,
            weight_limit_kg: None,
            height_limit_m: None,
            hazmat_restricted: false,
            length_m: 111.0,
            travel_time_s: Some(8.0),
        };
        let graph = RoadGraph {
            id: EntityId::new(),
            region: "tel-aviv".into(),
            nodes: vec![n1, n2],
            segments: vec![seg],
            version: 1,
            updated_at: Utc::now(),
        };
        RoadGraphIndex::from_graph(&graph)
    }

    #[test]
    fn match_near_road_returns_high_confidence() {
        let index = make_index();
        let mut matcher = MapMatcher::new();
        let pos = GeoPosition {
            latitude_deg: 32.0858,
            longitude_deg: 34.7818,
            altitude_m: None,
        };
        let result = matcher.match_position(&pos, &index);
        assert!(result.segment_id.is_some());
        assert!(result.offset_m < 5.0);
        assert_eq!(result.confidence, MatchConfidence::High);
    }

    #[test]
    fn match_far_from_road_returns_no_match() {
        let index = make_index();
        let mut matcher = MapMatcher::new();
        let pos = GeoPosition {
            latitude_deg: 32.09,
            longitude_deg: 34.79,
            altitude_m: None,
        };
        let result = matcher.match_position(&pos, &index);
        assert_eq!(result.confidence, MatchConfidence::NoMatch);
    }

    #[test]
    fn bearing_north_is_zero() {
        let a = GeoPosition {
            latitude_deg: 32.0,
            longitude_deg: 34.0,
            altitude_m: None,
        };
        let b = GeoPosition {
            latitude_deg: 33.0,
            longitude_deg: 34.0,
            altitude_m: None,
        };
        let bearing = bearing_deg(&a, &b);
        assert!(bearing.abs() < 1.0 || (360.0 - bearing).abs() < 1.0);
    }

    #[test]
    fn low_confidence_streak_increments() {
        let index = make_index();
        let mut matcher = MapMatcher::new();
        let far = GeoPosition {
            latitude_deg: 33.0,
            longitude_deg: 35.0,
            altitude_m: None,
        };
        for _ in 0..5 {
            matcher.match_position(&far, &index);
        }
        assert_eq!(matcher.low_confidence_streak(), 5);
    }
}
