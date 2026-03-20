//! Visual route guidance — turn-by-turn visual cues, lane arrows,
//! distance countdowns, and maneuver previews.

use serde::{Deserialize, Serialize};

/// Type of visual maneuver cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManeuverVisual {
    /// Straight ahead
    Straight,
    /// Slight left turn
    SlightLeft,
    /// Regular left turn
    Left,
    /// Sharp left turn
    SharpLeft,
    /// Slight right turn
    SlightRight,
    /// Regular right turn
    Right,
    /// Sharp right turn
    SharpRight,
    /// U-turn
    UTurn,
    /// Merge onto highway
    MergeOnto,
    /// Exit highway
    ExitHighway,
    /// Enter roundabout
    RoundaboutEnter,
    /// Exit roundabout at Nth exit
    RoundaboutExit,
    /// Arrive at destination
    Arrive,
    /// Arrive at waypoint
    ArriveWaypoint,
    /// Ferry crossing
    Ferry,
}

/// A visual turn-by-turn instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualInstruction {
    /// Maneuver type
    pub maneuver: ManeuverVisual,
    /// Distance to maneuver point (meters)
    pub distance_m: f64,
    /// Street name after the maneuver
    pub street_name: String,
    /// Roundabout exit number (if applicable)
    pub exit_number: Option<u8>,
    /// Verbal instruction text
    pub instruction_text: String,
    /// Whether this is the active (next) instruction
    pub is_active: bool,
    /// Lane configuration at this point
    pub lanes: Vec<LaneVisual>,
}

/// Visual lane indicator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneVisual {
    /// Lane direction options
    pub directions: Vec<ManeuverVisual>,
    /// Whether this lane is recommended
    pub recommended: bool,
    /// Whether this lane is highlighted (active guidance)
    pub highlighted: bool,
}

/// Distance display format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceUnit {
    Meters,
    Kilometers,
    Feet,
    Miles,
}

/// Format a distance for display.
pub fn format_distance(distance_m: f64, unit: DistanceUnit) -> String {
    match unit {
        DistanceUnit::Meters => {
            if distance_m < 1000.0 {
                format!("{:.0} m", distance_m)
            } else {
                format!("{:.1} km", distance_m / 1000.0)
            }
        }
        DistanceUnit::Kilometers => format!("{:.1} km", distance_m / 1000.0),
        DistanceUnit::Feet => format!("{:.0} ft", distance_m * 3.28084),
        DistanceUnit::Miles => {
            let miles = distance_m / 1609.34;
            if miles < 0.1 {
                format!("{:.0} ft", distance_m * 3.28084)
            } else {
                format!("{:.1} mi", miles)
            }
        }
    }
}

/// Visual guidance state for the current navigation session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidanceState {
    /// Upcoming instructions (next first)
    pub instructions: Vec<VisualInstruction>,
    /// Estimated time of arrival (formatted)
    pub eta: String,
    /// Remaining distance (meters)
    pub remaining_distance_m: f64,
    /// Remaining time (seconds)
    pub remaining_time_s: u64,
    /// Current speed (km/h)
    pub current_speed_kmh: f64,
    /// Speed limit at current position (km/h)
    pub speed_limit_kmh: Option<u32>,
    /// Whether the user is exceeding the speed limit
    pub over_speed_limit: bool,
    /// Current street name
    pub current_street: String,
    /// Distance unit preference
    pub distance_unit: DistanceUnit,
}

impl GuidanceState {
    /// Create an empty guidance state.
    pub fn empty() -> Self {
        Self {
            instructions: Vec::new(),
            eta: String::new(),
            remaining_distance_m: 0.0,
            remaining_time_s: 0,
            current_speed_kmh: 0.0,
            speed_limit_kmh: None,
            over_speed_limit: false,
            current_street: String::new(),
            distance_unit: DistanceUnit::Meters,
        }
    }

    /// Get the next upcoming instruction.
    pub fn next_instruction(&self) -> Option<&VisualInstruction> {
        self.instructions.iter().find(|i| i.is_active)
    }

    /// Format the remaining distance for display.
    pub fn formatted_remaining(&self) -> String {
        format_distance(self.remaining_distance_m, self.distance_unit)
    }

    /// Format ETA as remaining time.
    pub fn formatted_remaining_time(&self) -> String {
        let hours = self.remaining_time_s / 3600;
        let minutes = (self.remaining_time_s % 3600) / 60;
        if hours > 0 {
            format!("{hours}h {minutes}min")
        } else {
            format!("{minutes} min")
        }
    }

    /// Update speed limit status.
    pub fn update_speed_check(&mut self) {
        self.over_speed_limit = self
            .speed_limit_kmh
            .is_some_and(|limit| self.current_speed_kmh > f64::from(limit));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_distance_meters() {
        assert_eq!(format_distance(500.0, DistanceUnit::Meters), "500 m");
        assert_eq!(format_distance(1500.0, DistanceUnit::Meters), "1.5 km");
    }

    #[test]
    fn test_format_distance_miles() {
        let result = format_distance(1609.34, DistanceUnit::Miles);
        assert!(result.contains("mi"));

        let short = format_distance(10.0, DistanceUnit::Miles);
        assert!(short.contains("ft"));
    }

    #[test]
    fn test_guidance_state_remaining_time() {
        let mut state = GuidanceState::empty();
        state.remaining_time_s = 3900; // 1h 5min
        assert_eq!(state.formatted_remaining_time(), "1h 5min");

        state.remaining_time_s = 300; // 5min
        assert_eq!(state.formatted_remaining_time(), "5 min");
    }

    #[test]
    fn test_guidance_speed_check() {
        let mut state = GuidanceState::empty();
        state.speed_limit_kmh = Some(60);
        state.current_speed_kmh = 55.0;
        state.update_speed_check();
        assert!(!state.over_speed_limit);

        state.current_speed_kmh = 65.0;
        state.update_speed_check();
        assert!(state.over_speed_limit);
    }

    #[test]
    fn test_guidance_no_speed_limit() {
        let mut state = GuidanceState::empty();
        state.speed_limit_kmh = None;
        state.current_speed_kmh = 200.0;
        state.update_speed_check();
        assert!(!state.over_speed_limit);
    }

    #[test]
    fn test_next_instruction() {
        let mut state = GuidanceState::empty();
        state.instructions.push(VisualInstruction {
            maneuver: ManeuverVisual::Right,
            distance_m: 200.0,
            street_name: "Main St".to_string(),
            exit_number: None,
            instruction_text: "Turn right onto Main St".to_string(),
            is_active: true,
            lanes: vec![],
        });
        assert!(state.next_instruction().is_some());
        assert_eq!(
            state.next_instruction().unwrap().maneuver,
            ManeuverVisual::Right
        );
    }

    #[test]
    fn test_lane_visual() {
        let lane = LaneVisual {
            directions: vec![ManeuverVisual::Straight, ManeuverVisual::Right],
            recommended: true,
            highlighted: true,
        };
        assert!(lane.recommended);
        assert_eq!(lane.directions.len(), 2);
    }
}
