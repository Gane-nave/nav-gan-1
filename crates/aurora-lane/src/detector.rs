//! Lane detector — determines which lane the vehicle is currently in.

use aurora_core::map::{LaneGraph, LaneType};
use aurora_core::types::{EntityId, GeoPosition};
use aurora_map::graph::haversine_m;
use tracing::debug;

/// Result of a lane detection.
#[derive(Debug, Clone)]
pub struct LanePosition {
    /// ID of the detected lane.
    pub lane_id: EntityId,
    /// Lane index (0 = leftmost).
    pub lane_index: u8,
    /// Lane type.
    pub lane_type: LaneType,
    /// Lateral offset from lane centre (metres, positive = right).
    pub lateral_offset_m: f64,
    /// Confidence [0, 1].
    pub confidence: f64,
}

/// Detects the current lane based on position and lane graph data.
pub struct LaneDetector {
    /// Minimum confidence to report a lane detection.
    min_confidence: f64,
    /// Previous detection for temporal consistency.
    previous_detection: Option<LanePosition>,
    /// Lane change cooldown counter.
    cooldown_remaining: u32,
}

impl LaneDetector {
    pub fn new() -> Self {
        Self {
            min_confidence: 0.3,
            previous_detection: None,
            cooldown_remaining: 0,
        }
    }

    /// Detect which lane the vehicle is in based on position.
    pub fn detect(
        &mut self,
        position: &GeoPosition,
        lane_graph: &LaneGraph,
    ) -> Option<LanePosition> {
        if lane_graph.lanes.is_empty() {
            return None;
        }

        // Decrement cooldown.
        if self.cooldown_remaining > 0 {
            self.cooldown_remaining -= 1;
        }

        // Find the closest lane.
        let mut best_lane: Option<(usize, f64)> = None;

        for (i, lane) in lane_graph.lanes.iter().enumerate() {
            let min_dist = lane
                .geometry
                .iter()
                .map(|p| haversine_m(position, p))
                .fold(f64::INFINITY, f64::min);

            if best_lane.is_none() || min_dist < best_lane.unwrap().1 {
                best_lane = Some((i, min_dist));
            }
        }

        let (lane_idx, dist) = best_lane?;
        let lane = &lane_graph.lanes[lane_idx];

        // Compute confidence based on distance.
        let lane_width = lane.width_m.unwrap_or(3.5);
        let confidence = if dist < lane_width / 2.0 {
            1.0
        } else if dist < lane_width {
            0.7
        } else if dist < lane_width * 2.0 {
            0.4
        } else {
            0.1
        };

        if confidence < self.min_confidence {
            return None;
        }

        // Temporal consistency: if same lane as before, boost confidence.
        let confidence = if let Some(prev) = &self.previous_detection {
            if prev.lane_index == lane.index {
                (confidence + 0.1).min(1.0)
            } else if self.cooldown_remaining > 0 {
                // During cooldown, stick with previous lane.
                debug!("lane change cooldown active — keeping previous lane");
                return self.previous_detection.clone();
            } else {
                // Lane change detected — apply cooldown.
                self.cooldown_remaining = 3;
                confidence
            }
        } else {
            confidence
        };

        let detection = LanePosition {
            lane_id: lane.id,
            lane_index: lane.index,
            lane_type: lane.lane_type,
            lateral_offset_m: dist,
            confidence,
        };

        debug!(
            lane_index = detection.lane_index,
            offset_m = detection.lateral_offset_m,
            confidence = detection.confidence,
            "lane detected"
        );

        self.previous_detection = Some(detection.clone());
        Some(detection)
    }

    /// Reset the detector state.
    pub fn reset(&mut self) {
        self.previous_detection = None;
        self.cooldown_remaining = 0;
    }

    /// Last detection result.
    pub fn last_detection(&self) -> Option<&LanePosition> {
        self.previous_detection.as_ref()
    }
}

impl Default for LaneDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::map::{Lane, TurnDirection};
    use chrono::Utc;

    fn make_lane_graph() -> LaneGraph {
        let left_lane = Lane {
            id: EntityId::new(),
            index: 0,
            lane_type: LaneType::Driving,
            width_m: Some(3.5),
            geometry: vec![
                GeoPosition {
                    latitude_deg: 32.085,
                    longitude_deg: 34.7815,
                    altitude_m: None,
                },
                GeoPosition {
                    latitude_deg: 32.086,
                    longitude_deg: 34.7815,
                    altitude_m: None,
                },
            ],
            allowed_turns: vec![TurnDirection::Straight, TurnDirection::Left],
            speed_limit_kmh: Some(50.0),
        };
        let right_lane = Lane {
            id: EntityId::new(),
            index: 1,
            lane_type: LaneType::Driving,
            width_m: Some(3.5),
            geometry: vec![
                GeoPosition {
                    latitude_deg: 32.085,
                    longitude_deg: 34.7818,
                    altitude_m: None,
                },
                GeoPosition {
                    latitude_deg: 32.086,
                    longitude_deg: 34.7818,
                    altitude_m: None,
                },
            ],
            allowed_turns: vec![TurnDirection::Straight, TurnDirection::Right],
            speed_limit_kmh: Some(50.0),
        };

        LaneGraph {
            id: EntityId::new(),
            segment_id: EntityId::new(),
            lanes: vec![left_lane, right_lane],
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn detects_correct_lane() {
        let mut detector = LaneDetector::new();
        let graph = make_lane_graph();

        // Position close to right lane (within ~1m of geometry point).
        let pos = GeoPosition {
            latitude_deg: 32.08501,
            longitude_deg: 34.7818,
            altitude_m: None,
        };
        let result = detector.detect(&pos, &graph);
        assert!(result.is_some());
        let lane = result.unwrap();
        assert_eq!(lane.lane_index, 1); // right lane
    }

    #[test]
    fn no_detection_far_from_lanes() {
        let mut detector = LaneDetector::new();
        let graph = make_lane_graph();

        let far = GeoPosition {
            latitude_deg: 33.0,
            longitude_deg: 35.0,
            altitude_m: None,
        };
        let result = detector.detect(&far, &graph);
        assert!(result.is_none());
    }

    #[test]
    fn temporal_consistency_boosts_confidence() {
        let mut detector = LaneDetector::new();
        let graph = make_lane_graph();

        let pos = GeoPosition {
            latitude_deg: 32.08501,
            longitude_deg: 34.7818,
            altitude_m: None,
        };

        let first = detector.detect(&pos, &graph).unwrap();
        let second = detector.detect(&pos, &graph).unwrap();
        // Second detection on same lane should have equal or higher confidence.
        assert!(second.confidence >= first.confidence);
    }
}
