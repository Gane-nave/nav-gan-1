//! Lane advisor — recommends lane changes for upcoming maneuvers.

use gane_core::map::{Lane, LaneGraph, LaneType, TurnDirection};
use gane_core::route::{Maneuver, ManeuverType};
use tracing::debug;

/// A lane change recommendation.
#[derive(Debug, Clone)]
pub struct LaneRecommendation {
    /// Recommended lane index.
    pub target_lane_index: u8,
    /// Reason for the recommendation.
    pub reason: String,
    /// Urgency level (0.0 = no rush, 1.0 = must change now).
    pub urgency: f64,
    /// Distance to the point where lane matters (metres).
    pub distance_m: f64,
}

/// Advises on optimal lane positioning for upcoming maneuvers.
pub struct LaneAdvisor {
    /// Distance threshold for early lane change recommendation (metres).
    early_warning_distance_m: f64,
    /// Distance threshold for urgent lane change (metres).
    urgent_distance_m: f64,
}

impl LaneAdvisor {
    pub fn new() -> Self {
        Self {
            early_warning_distance_m: 500.0,
            urgent_distance_m: 100.0,
        }
    }

    /// Set the early warning distance.
    pub fn with_early_warning(mut self, distance_m: f64) -> Self {
        self.early_warning_distance_m = distance_m;
        self
    }

    /// Recommend a lane based on upcoming maneuver and current lane graph.
    pub fn recommend(
        &self,
        current_lane_index: u8,
        distance_to_maneuver_m: f64,
        next_maneuver: &Maneuver,
        lane_graph: &LaneGraph,
    ) -> Option<LaneRecommendation> {
        if distance_to_maneuver_m > self.early_warning_distance_m {
            return None; // Too far — no recommendation needed.
        }

        let required_turn = maneuver_to_turn(next_maneuver);
        let required_turn = required_turn?;

        // Find lanes that allow the required turn.
        let valid_lanes: Vec<&Lane> = lane_graph
            .lanes
            .iter()
            .filter(|l| {
                l.lane_type == LaneType::Driving && l.allowed_turns.contains(&required_turn)
            })
            .collect();

        if valid_lanes.is_empty() {
            return None;
        }

        // Check if current lane is already valid.
        let current_is_valid = valid_lanes.iter().any(|l| l.index == current_lane_index);
        if current_is_valid {
            return None; // Already in a good lane.
        }

        // Find the closest valid lane.
        let target = valid_lanes
            .iter()
            .min_by_key(|l| (l.index as i16 - current_lane_index as i16).unsigned_abs())
            .unwrap();

        let urgency = if distance_to_maneuver_m < self.urgent_distance_m {
            1.0
        } else {
            1.0 - (distance_to_maneuver_m - self.urgent_distance_m)
                / (self.early_warning_distance_m - self.urgent_distance_m)
        };

        let direction = if target.index > current_lane_index {
            "right"
        } else {
            "left"
        };

        debug!(
            current = current_lane_index,
            target = target.index,
            urgency,
            distance_m = distance_to_maneuver_m,
            "lane recommendation"
        );

        Some(LaneRecommendation {
            target_lane_index: target.index,
            reason: format!(
                "Move {} for upcoming {:?}",
                direction, next_maneuver.maneuver_type
            ),
            urgency: urgency.clamp(0.0, 1.0),
            distance_m: distance_to_maneuver_m,
        })
    }

    /// Get lane arrows for display (which lanes are valid for the turn).
    pub fn lane_arrows(&self, next_maneuver: &Maneuver, lane_graph: &LaneGraph) -> Vec<LaneArrow> {
        let required_turn = maneuver_to_turn(next_maneuver);

        lane_graph
            .lanes
            .iter()
            .filter(|l| l.lane_type == LaneType::Driving)
            .map(|l| {
                let is_recommended = required_turn
                    .as_ref()
                    .map(|t| l.allowed_turns.contains(t))
                    .unwrap_or(true);

                LaneArrow {
                    lane_index: l.index,
                    arrows: l.allowed_turns.iter().map(|t| format!("{t:?}")).collect(),
                    highlighted: is_recommended,
                }
            })
            .collect()
    }
}

impl Default for LaneAdvisor {
    fn default() -> Self {
        Self::new()
    }
}

/// Visual representation of a lane arrow for UI rendering.
#[derive(Debug, Clone)]
pub struct LaneArrow {
    pub lane_index: u8,
    pub arrows: Vec<String>,
    pub highlighted: bool,
}

/// Map a maneuver type to the required turn direction.
fn maneuver_to_turn(maneuver: &Maneuver) -> Option<TurnDirection> {
    match maneuver.maneuver_type {
        ManeuverType::TurnLeft | ManeuverType::TurnSharpLeft => Some(TurnDirection::Left),
        ManeuverType::TurnSlightLeft => Some(TurnDirection::SlightLeft),
        ManeuverType::TurnRight | ManeuverType::TurnSharpRight => Some(TurnDirection::Right),
        ManeuverType::TurnSlightRight => Some(TurnDirection::SlightRight),
        ManeuverType::UTurn => Some(TurnDirection::UTurn),
        ManeuverType::Continue | ManeuverType::Depart | ManeuverType::Arrive => {
            Some(TurnDirection::Straight)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use gane_core::types::{EntityId, GeoPosition};

    fn make_3lane_graph() -> LaneGraph {
        let lanes = (0..3)
            .map(|i| Lane {
                id: EntityId::new(),
                index: i,
                lane_type: LaneType::Driving,
                width_m: Some(3.5),
                geometry: vec![GeoPosition {
                    latitude_deg: 32.085,
                    longitude_deg: 34.781 + i as f64 * 0.0003,
                    altitude_m: None,
                }],
                allowed_turns: if i == 0 {
                    vec![TurnDirection::Left, TurnDirection::Straight]
                } else if i == 2 {
                    vec![TurnDirection::Right, TurnDirection::Straight]
                } else {
                    vec![TurnDirection::Straight]
                },
                speed_limit_kmh: Some(50.0),
            })
            .collect();

        LaneGraph {
            id: EntityId::new(),
            segment_id: EntityId::new(),
            lanes,
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn recommends_left_lane_for_left_turn() {
        let advisor = LaneAdvisor::new();
        let graph = make_3lane_graph();

        let maneuver = Maneuver {
            position: GeoPosition {
                latitude_deg: 32.086,
                longitude_deg: 34.781,
                altitude_m: None,
            },
            maneuver_type: ManeuverType::TurnLeft,
            instruction: "Turn left".into(),
            distance_to_m: 200.0,
            street_name: None,
            lane_guidance: None,
        };

        // Currently in lane 2 (rightmost), need to go left.
        let rec = advisor.recommend(2, 200.0, &maneuver, &graph);
        assert!(rec.is_some());
        let rec = rec.unwrap();
        assert_eq!(rec.target_lane_index, 0);
    }

    #[test]
    fn no_recommendation_when_already_in_correct_lane() {
        let advisor = LaneAdvisor::new();
        let graph = make_3lane_graph();

        let maneuver = Maneuver {
            position: GeoPosition {
                latitude_deg: 32.086,
                longitude_deg: 34.781,
                altitude_m: None,
            },
            maneuver_type: ManeuverType::TurnLeft,
            instruction: "Turn left".into(),
            distance_to_m: 200.0,
            street_name: None,
            lane_guidance: None,
        };

        // Already in lane 0 (leftmost).
        let rec = advisor.recommend(0, 200.0, &maneuver, &graph);
        assert!(rec.is_none());
    }

    #[test]
    fn no_recommendation_when_far_away() {
        let advisor = LaneAdvisor::new();
        let graph = make_3lane_graph();

        let maneuver = Maneuver {
            position: GeoPosition {
                latitude_deg: 32.086,
                longitude_deg: 34.781,
                altitude_m: None,
            },
            maneuver_type: ManeuverType::TurnLeft,
            instruction: "Turn left".into(),
            distance_to_m: 1000.0,
            street_name: None,
            lane_guidance: None,
        };

        let rec = advisor.recommend(2, 1000.0, &maneuver, &graph);
        assert!(rec.is_none());
    }

    #[test]
    fn lane_arrows_highlights_correct_lanes() {
        let advisor = LaneAdvisor::new();
        let graph = make_3lane_graph();

        let maneuver = Maneuver {
            position: GeoPosition {
                latitude_deg: 32.086,
                longitude_deg: 34.781,
                altitude_m: None,
            },
            maneuver_type: ManeuverType::TurnRight,
            instruction: "Turn right".into(),
            distance_to_m: 200.0,
            street_name: None,
            lane_guidance: None,
        };

        let arrows = advisor.lane_arrows(&maneuver, &graph);
        assert_eq!(arrows.len(), 3);
        assert!(!arrows[0].highlighted); // left lane — no right turn
        assert!(!arrows[1].highlighted); // middle — no right turn
        assert!(arrows[2].highlighted); // right lane — has right turn
    }
}
